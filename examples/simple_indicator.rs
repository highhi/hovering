use hovering::Indicator;
use std::thread;
use std::time::Duration;

fn main() {
    let total = 100;
    let mut indicator = Indicator::new(total);

    for _ in 0..=total {
        indicator.increment();
        thread::sleep(Duration::from_millis(100));
    }
    println!("\nDone!");
}
