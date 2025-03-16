use avatar_rs::Icon;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let icon = Icon::new(1253);
    icon.save("test.png").unwrap();
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
}
