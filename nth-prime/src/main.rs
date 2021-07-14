use nth_prime::*;
use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};

fn main() {
    let n = 1_00_000;
    let durations = (0..=n)
        .into_par_iter()
        .map(nthprimetime)
        .collect::<Vec<(u32, u32, Duration)>>();
    let mut f = File::create("output.txt").expect("Unable to create file");
    for i in durations {
        writeln!(
            f,
            "nth-prime {}, value {}, elasped-time {:?}",
            i.0, i.1, i.2
        )
        .expect("Unable to create file");
    }
}

fn nthprimetime(n: u32) -> (u32, u32, Duration) {
    let start = Instant::now();
    let val = nth(n);
    let elapsed = start.elapsed();
    // println!("nth-prime {}, value {}, elasped-time {:?}", n, val, elapsed);
    (n, val, elapsed)
}
