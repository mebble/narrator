use std::{thread, time};
use std::io::{self, stdout, Write};

fn main() -> io::Result<()> {
    let duration = time::Duration::from_millis(3000);
    let now = time::Instant::now();

    print!("Gonna sleep now, world...\n");
    stdout().flush()?;

    thread::sleep(duration);
    assert!(now.elapsed() >= duration);

    print!("Good morning, world!\n");

    Ok(())
}
