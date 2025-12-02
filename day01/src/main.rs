use std::fs;
const INPUT_FILE: &'static str = "../input/input_day_01.txt";
fn main() {
    let _ = day1_part_one();
    let _ = day1_part_two();
}

fn day1_part_one() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let mut dial: i32 = 50;
    let mut zenith = 0;
    for (line_number, rotation) in input.trim().lines().enumerate() {
        let (direction, ticks) = rotation.split_at(1);
        let rotation: i32 = match direction {
            "R" => ticks
                .parse()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
            "L" => -ticks
                .parse()
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "Invalid rotation: {} at line number: {}",
                        input, line_number
                    ),
                ));
            }
        };
        dial = (dial + rotation) % 100;
        if dial == 0 {
            zenith += 1;
        }
    }
    println!("part one zeniths: {}", zenith);
    Ok(())
}

fn day1_part_two() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let mut safe = Safe::new(50);
    for (line_number, rotation) in input.trim().lines().enumerate() {
        let (direction, ticks) = rotation.split_at(1);
        match direction {
            "R" => {
                let ticks = ticks
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                safe.rotate_right(ticks);
            }
            "L" => {
                let ticks = ticks
                    .parse()
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                safe.rotate_left(ticks);
            }
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "Invalid rotation: {} at line number: {}",
                        input, line_number
                    ),
                ));
            }
        };
    }
    println!("part two clicks: {}", safe.clicks);
    Ok(())
}

#[derive(Debug)]
struct Safe {
    dial: usize,
    clicks: usize,
}

impl Safe {
    fn new(dial: usize) -> Self {
        Self { dial, clicks: 0 }
    }
    fn rotate_left(self: &mut Self, ticks: usize) {
        let mut clicks = ticks / 100;
        let rotate = ticks % 100;
        if rotate >= self.dial {
            if self.dial > 0 {
                clicks += 1;
            }
            self.dial = (100 - (rotate - self.dial)) % 100;
        } else {
            self.dial -= rotate;
        }
        self.clicks += clicks;
    }
    fn rotate_right(self: &mut Self, ticks: usize) {
        if (self.dial + ticks) >= 100 {
            self.clicks += (self.dial + ticks) / 100;
        }
        self.dial = (self.dial + ticks) % 100;
    }
}
