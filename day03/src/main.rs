use std::fs;

const INPUT_FILE: &'static str = "../input/input_day_03.txt";

fn main() {
    if let Err(error) = day3_part_one() {
        eprintln!("error: {}", error);
    }
}

#[derive(Clone, Debug)]
struct Battery {
    pos: usize,
    jolt: char,
}

impl Battery {
    fn new(pos: usize, jolt: char) -> Self {
        Self { pos, jolt }
    }
}

fn day3_part_one() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut total_joltage: u64 = 0;
    for (line_number, line) in input.trim().lines().enumerate() {
        let line = line.trim();
        let Some(mut max1) = line
            .chars()
            .enumerate()
            .next()
            .and_then(|(pos, jolt)| Some(Battery::new(pos, jolt)))
        else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid battery bank at line: {}", line_number),
            ));
        };

        let Some(last_battery) = line
            .chars()
            .enumerate()
            .last()
            .and_then(|(pos, jolt)| Some(Battery::new(pos, jolt)))
        else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid battery bank at line: {}", line_number),
            ));
        };
        let mut max2 = last_battery.clone();

        for (pos, jolt) in line
            .chars()
            .enumerate()
            .skip(1)
            .take(line.len().saturating_sub(2))
        {
            if jolt > max1.jolt {
                (max1.pos, max1.jolt) = (pos, jolt);
                max2 = last_battery.clone();
            } else if jolt > max2.jolt {
                (max2.pos, max2.jolt) = (pos, jolt);
            }
        }
        let battery_jolt: u64 = format!("{}{}", max1.jolt, max2.jolt)
            .parse()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        println!("{}", battery_jolt);
        total_joltage += battery_jolt;
    }
    println!("total joltage: {}", total_joltage);
    Ok(())
}
