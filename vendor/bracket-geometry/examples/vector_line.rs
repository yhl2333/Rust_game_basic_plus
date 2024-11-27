use bracket_geometry::prelude::*;
use crossterm::queue;
use crossterm::style::Print;
use std::io::{stdout, Write};

fn main() {
    let mut fake_console: Vec<char> = vec!['.'; 100];
    for point in VectorLine::new(Point::new(1, 1), Point::new(7, 9)) {
        let idx = ((point.y * 10) + point.x) as usize;
        fake_console[idx] = '*';
    }

    for y in 0..10 {
        let mut line = String::from("");
        let idx = y * 10;
        for x in 0..10 {
            line.push(fake_console[idx + x]);
        }
        line.push('\n');
        queue!(stdout(), Print(&line)).expect("Command fail");
    }
    stdout().flush().expect("Flush Fail");
}
