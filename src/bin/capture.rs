use std::process::Command;

fn date() -> String {
    let date = Command::new("date").output().unwrap();
    std::str::from_utf8(&date.stdout).unwrap().trim().to_owned()
}

fn log(msg: &str) {
    println!("{}: {msg}", date());
}

fn elog(msg: &str) {
    eprintln!("{}: {msg}", date());
}

// 1st field: The size of the allocated memory.
// 2nd field: The size of the allocated physical memory.
// 3rd field: The number of the major fault.
// 4th field: The number of the minor fault.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pgrep = Command::new("pgrep").arg("demand-paging").output()?;

    let pids = match pgrep.status.code().expect("failed to execute pgrep") {
        0 => std::str::from_utf8(&pgrep.stdout)?,
        1 => panic!("demand-paging process does not exist"),
        2 => unreachable!("pgrep syntax error: {pgrep:?}"),
        3 => panic!("fatal error: {pgrep:?}"),
        rc => unreachable!("pgrep does not return this error code: {rc}"),
    };

    let pids: Vec<&str> = pids.trim().split('\n').collect();

    if pids.len() != 1 {
        panic!("multiple demand-paging processes exist: {pids:?}");
    }

    let pid = pids[0];

    loop {
        let ps = Command::new("ps")
            .args(["-h", "-o", "vsz,rss,maj_flt,min_flt", "-p", pid])
            .output()?;
        if !ps.status.success() {
            elog("demand-paging process finished");
            std::process::exit(1);
        }
        let info = std::str::from_utf8(&ps.stdout)?.trim();
        log(info);
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
