use std::{
    fs::File,
    io::{self, BufWriter, Write},
    time::Instant,
};

const CONCURRENCY: usize = 2;
const NLOOP_FOR_ESTIMATION: u128 = 100000000;

fn estimate_loops_per_msec() -> u128 {
    let before = Instant::now();
    for _ in 0..NLOOP_FOR_ESTIMATION {}
    NLOOP_FOR_ESTIMATION / before.elapsed().as_millis()
}

fn child_fn(n: usize, nloop_per_msec: u128, instant: &Instant) -> io::Result<()> {
    let progress = {
        let mut progress = [None; 100];
        for item in &mut progress {
            for _ in 0..nloop_per_msec {}
            *item = Some(instant.elapsed());
        }
        progress.into_iter().flatten()
    };

    let mut fd = BufWriter::new(File::create(format!("{n}.data"))?);
    for (i, duration) in progress.enumerate() {
        fd.write_all(format!("{}\t{i}\n", duration.as_millis()).as_bytes())?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();

    let nice: i32 = args
        .nth(1)
        .expect("this program must be called with one argument")
        .parse()
        .unwrap_or_else(|e| panic!("failed to parse the argument as number: {e}"));

    unsafe {
        let mut set = std::mem::zeroed();
        libc::CPU_SET(0, &mut set);
        libc::sched_setaffinity(0, std::mem::size_of_val(&set), &set);
    };

    let mut forked = Vec::with_capacity(CONCURRENCY);
    let nloop_per_msec = estimate_loops_per_msec();
    let start = Instant::now();

    for i in 0..CONCURRENCY {
        let pid: u32 = unsafe { libc::fork() }
            .try_into()
            .expect("failed to fork process");
        if pid == 0 {
            if i == CONCURRENCY - 1 && unsafe { libc::nice(nice) } == -1 {
                panic!("failed to set nice");
            }
            child_fn(i, nloop_per_msec, &start)?;
            return Ok(());
        } else {
            forked.push(pid);
        }
    }

    let mut status = 0;
    for pid in forked {
        let _ = unsafe { libc::waitpid(pid as i32, &mut status, 0) };
        assert!(
            libc::WIFEXITED(status),
            "child process `{pid}` did not terminate normally"
        );
    }

    Ok(())
}
