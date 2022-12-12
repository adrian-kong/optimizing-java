use std::error::Error;
use std::ops::Sub;
use std::time::{SystemTime, UNIX_EPOCH};

const SIZE: usize = 2 * 1024 * 1024;
const STEP: usize = 1;
// const RUNS: usize = 5;

// fn run(step: usize) -> Result<u128, Box<dyn Error>> {
//     let mut time_sum = 0;
//
//     for runs in 0..RUNS {
//         let mut array = vec![0; SIZE].into_boxed_slice();
//         let start = SystemTime::now().duration_since(UNIX_EPOCH)?;
//         for i in (0..SIZE).step_by(step) {
//             array[i] += 1;
//         }
//         let time_elapsed = SystemTime::now().sub(start).duration_since(UNIX_EPOCH)?.as_nanos();
//         time_sum += time_elapsed;
//     }
//
//     Ok(time_sum / RUNS as u128)
// }

fn touch_line(array: &mut [u32]) {
    for i in (0..SIZE).step_by(STEP) {
        array[i] += 1;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // warmup
    let mut array = vec![0_u32; SIZE].into_boxed_slice();
    let start = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    println!("START: {start}");
    for _ in 0..15_000 {
        touch_line(&mut array);
    }
    let time_elapsed = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() - start;
    println!("WARMUP FINISHED: {time_elapsed}");
    for _ in 0..100 {
        let start = SystemTime::now().duration_since(UNIX_EPOCH)?;
        touch_line(&mut array);
        let time_elapsed = SystemTime::now().sub(start).duration_since(UNIX_EPOCH)?.as_nanos();
        println!("{time_elapsed}");
    }
    Ok(())
}
