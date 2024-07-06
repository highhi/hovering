use std::io::{self, Write};

pub struct Indicator {
    total: usize,
    current: usize,
}

impl Indicator {
    pub fn new(total: usize) -> Self {
        Indicator { total, current: 0 }
    }

    pub fn increment(&mut self) {
        if self.current < self.total {
            self.current += 1;
            self.print_progress();
        }

        if self.current == self.total {
            self.clear_progress();
        }
    }

    pub fn print_progress(&self) {
        let percent = (self.current as f32 / self.total as f32) * 100.0;
        let filled = (percent / 2.0) as usize;
        let empty = 50 - filled;

        print!("\r[");
        for _ in 0..filled {
            print!("#");
        }
        for _ in 0..empty {
            print!(" ");
        }
        print!("] {:.1}%", percent);
        io::stdout().flush().unwrap();
    }

    fn clear_progress(&self) {
        print!("\r");
        for _ in 0..100 {
            print!(" ");
        }
        print!("\r");
        io::stdout().flush().unwrap();
    }
}
