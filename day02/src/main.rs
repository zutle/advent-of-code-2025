use std::fs;

const INPUT_FILE: &'static str = "../input/input_day_02.txt";

fn main() {
    // if let Err(error) = day2_part_one() {
    //     eprintln!("error: {}", error);
    // }
    println!("start");
    if let Err(error) = day2_part_two() {
        eprintln!("error: {}", error);
    }
    // test();
}

#[allow(dead_code)]
fn day2_part_one() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;

    let mut sum_invalid_ids: u64 = 0;
    for range in input.trim().split(",") {
        if let Some((start, end)) = range.split_once("-") {
            sum_invalid_ids += find_duplicates_part_one(start, end)?;
        }
    }

    println!("sum invalid ids: {}", sum_invalid_ids);
    Ok(())
}

#[allow(dead_code)]
fn find_duplicates_part_one(start: &str, end: &str) -> std::io::Result<u64> {
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

fn day2_part_two() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    // let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let mut sum_invalid_ids: u64 = 0;
    for range in input.trim().split(",") {
        if let Some((start, end)) = range.trim().split_once("-") {
            sum_invalid_ids += find_duplicates_part_two(start, end)?;
        }
    }

    println!("sum invalid ids: {}", sum_invalid_ids);
    Ok(())
}

fn find_duplicates_part_two(start: &str, end: &str) -> std::io::Result<u64> {
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
        if num_digits < 2 {
            continue;
        }
        let mut all_equal = true;
        if let Some(first_char) = id_string.chars().nth(0) {
            for c in id_string.chars().skip(1) {
                if c != first_char {
                    all_equal = false;
                    break;
                }
            }
        }
        if all_equal {
            println!("invalid: {}", id);
            sum_invalid_ids += id;
            continue;
        }

        // If all digits are not equal, the string length must be at least 4 to be invalid
        if num_digits < 4 {
            continue;
        }

        for split_into in 2..=id_string.len() / 2 {
            if is_invalid(&id_string, split_into) {
                println!("invalid: {}", id_string);
                sum_invalid_ids += id;
                continue;
            }
        }
    }
    Ok(sum_invalid_ids)
}

fn is_invalid(id: &str, split_into: usize) -> bool {
    // println!("id: {}", id);
    // println!("split into: {}", split_into);
    if id.len() % split_into != 0 {
        return false;
    }
    let chunk_size = id.len() / split_into;
    let first_chunk = &id[0..chunk_size];
    // println!("chunk size: {}", chunk_size);
    for chunk_num in 1..split_into {
        // println!("comparing {} to {}", first_chunk, &id[chunk_num*chunk_size..(chunk_num+1)*chunk_size]);
        if &id[chunk_num * chunk_size..(chunk_num + 1) * chunk_size] != first_chunk {
            return false;
        }
    }
    true
}

fn test() -> bool {
    let id = "828828828";
    for split_into in 2..=id.len() / 2 {
        if is_invalid(id, split_into) {
            println!("invalid: {}", id);
            return true;
        }
    }
    false
}
