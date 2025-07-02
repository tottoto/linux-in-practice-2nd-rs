NR<=nproc { sum_tat+=$1 }
NR==nproc+1 { total_real=$1 }
END { printf("%d\t%.3f\t%.3f\n", nproc, sum_tat/nproc, nproc/total_real) }
