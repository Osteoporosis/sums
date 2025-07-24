use std::time::Instant;

use sums::{
    chunked_sum, expanded_fold_sum, fold_sum, for_sum, iter_sum, wide_sum_fold0, wide_sum_fold1,
    wide_sum_fold2,
};

// const ITER: i32 = 20_000;
// const N: i32 = BLOCK * BLOCK / 2 - 1;

// const ITER: i32 = 20_000;
// const N: i32 = BLOCK * BLOCK / 2;

// const ITER: i32 = 20_000;
// const N: i32 = BLOCK * BLOCK / 2 + 1;

const ITER: i32 = 5;
const N: i32 = 200_000_000;

fn main() {
    // info
    use std::process::Command;
    let rustc_cpus = Command::new("rustc")
        .args(["--print", "target-cpus"])
        .output()
        .expect("failed to run rustc");
    let cpus_txt = String::from_utf8_lossy(&rustc_cpus.stdout);
    let cpu = cpus_txt
        .lines()
        .find_map(|l| l.split_once("currently ").map(|(_, right)| right))
        .and_then(|s| s.split_once(')').map(|(cpu, _)| cpu.trim()))
        .unwrap_or("unknown");
    println!("CPU: {}", cpu);

    let rustc_features = Command::new("rustc")
        .args(["-C", "target-cpu=native", "--print", "cfg"])
        .output()
        .expect("failed to run rustc");
    let features_txt = String::from_utf8_lossy(&rustc_features.stdout);
    let mut features: Vec<_> = features_txt
        .lines()
        .filter_map(|l| {
            l.strip_prefix("target_feature=\"")
                .and_then(|r| r.strip_suffix("\""))
        })
        .collect();
    features.sort();
    println!("Features: {}", features.join(", "));

    let v: Vec<f64> = {
        let values = 1..=N;
        println!(
            "correct_acc: {}",
            values.clone().map(|x| x as u128).sum::<u128>() * ITER as u128
        );
        values.rev().map(|x| x as f64).collect()
    };

    // bench: for_sum
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += for_sum(&v);
    }
    let dur = start.elapsed();
    println!("for_sum: {:?} (acc = {})", dur, acc);

    // bench: iter_sum
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += iter_sum(&v);
    }
    let dur = start.elapsed();
    println!("iter_sum: {:?} (acc = {})", dur, acc);

    // bench: fold_sum
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += fold_sum(&v);
    }
    let dur = start.elapsed();
    println!("fold_sum: {:?} (acc = {})", dur, acc);

    // bench: chunked_sum
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += chunked_sum(&v);
    }
    let dur = start.elapsed();
    println!("chunked_sum: {:?} (acc = {})", dur, acc);

    // bench: wide_sum_fold0
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += wide_sum_fold0(&v);
    }
    let dur = start.elapsed();
    println!("wide_sum_fold0: {:?} (acc = {})", dur, acc);

    // bench: wide_sum_fold1
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += wide_sum_fold1(&v);
    }
    let dur = start.elapsed();
    println!("wide_sum_fold1: {:?} (acc = {})", dur, acc);

    // bench: wide_sum_fold2
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += wide_sum_fold2(&v);
    }
    let dur = start.elapsed();
    println!("wide_sum_fold2: {:?} (acc = {})", dur, acc);

    // bench: expanded_fold_sum
    let start = Instant::now();
    let mut acc = 0.0f64;
    for _ in 0..ITER {
        acc += expanded_fold_sum(&v);
    }
    let dur = start.elapsed();
    println!("expanded_fold_sum: {:?} (acc = {})", dur, acc);
}
