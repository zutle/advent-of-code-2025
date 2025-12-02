use std::fs;

const INPUT_FILE: &'static str = "../input/input_day_02.txt";

fn main() {
    if let Err(error) = day2_part_one() {
        eprintln!("error: {}", error);
    }
}

fn day2_part_one() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut sum_invalid_ids: u64 = 0;
    for range in input.trim().split(",") {
        if let Some((start, end)) = range.split_once("-") {
            sum_invalid_ids += find_duplicates(start, end)?;
        }
    }

    println!("sum invalid ids: {}", sum_invalid_ids);
    Ok(())
}

fn find_duplicates(start: &str, end: &str) -> std::io::Result<u64> {
    let start: u64 = start
        .parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    let end: u64 = end
        .parse()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut sum_invalid_ids = 0;
    for id in start..=end {
        let id_string = id.to_string();
        let num_digits = id_string.len();
        if num_digits % 2 != 0 {
            continue;
        };

        let (first, second) = id_string.split_at(num_digits / 2);
        if first == second {
            sum_invalid_ids += id;
        }
    }
    Ok(sum_invalid_ids)
}
