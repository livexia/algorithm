// use rand::prelude::*;
use tokio::runtime;

fn main() -> std::io::Result<()> {
    let thread_rt = runtime::Runtime::new()?;
    for step in 10..1000000 {
        thread_rt.spawn(async move { run(step) });
    }
    Ok(())
}

fn run(l: u32) {
    // let mut rng = thread_rng();

    let loop_length = l;

    let fast_speed = 2;
    // let fast_speed = rng.gen_range(0..loop_length / 2);
    let slow_speed = 1;

    let mut fast_location = 0;
    let mut slow_location = 0;

    for step in 0..u32::MAX {
        fast_location = (fast_location + fast_speed) % loop_length;
        slow_location = (slow_location + slow_speed) % loop_length;
        if fast_location == slow_location {
            if step + 1 != loop_length {
                println!("Test with");
                println!("  - loop length: {}", loop_length);
                println!("  - fast speed : {}", fast_speed);
                println!("location : {}", fast_location);
                println!("take step: {}", step);
            }
            return;
        }
    }
}
