use std::io::{self, Write};
use std::time::Instant;

pub struct ProgressBar {
    total: usize,
    current: usize,
    start_time: Instant,
}

impl ProgressBar {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            current: 0,
            start_time: Instant::now(),
        }
    }

    pub fn increment(&mut self) {
        self.current += 1;
        self.display();
    }

    fn display(&self) {
        let percentage = (self.current as f64 / self.total as f64 * 100.0) as usize;
        let elapsed = self.start_time.elapsed().as_secs_f64();
        let items_per_second = self.current as f64 / elapsed;

        print!("\r[");
        for i in 0..50 {
            if i < percentage / 2 {
                print!("=");
            } else if i == percentage / 2 {
                print!(">");
            } else {
                print!(" ");
            }
        }
        print!("] {}/{} ({:.1}%) {:.1} files/s",
               self.current,
               self.total,
               percentage as f64,
               items_per_second
        );
        io::stdout().flush().unwrap();
    }
}

impl Drop for ProgressBar {
    fn drop(&mut self) {
        println!();
    }
}