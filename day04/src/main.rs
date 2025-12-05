use std::fs;

const INPUT_FILE: &'static str = "../input/input_day_04.txt";

fn main() {
    if let Err(error) = day4_part_one() {
        eprintln!("error: {}", error);
    }
}

fn day4_part_one() -> std::io::Result<()> {
    let input = fs::read_to_string(INPUT_FILE)?;
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim_end().chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid.first().map(|row| row.len()).unwrap_or(0);

    let mut accessible = 0usize;

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != '@' {
                continue;
            }

            let mut neighbors = 0u8;
            for dr in -1i32..=1 {
                for dc in -1i32..=1 {
                    if dr == 0 && dc == 0 {
                        continue;
                    }

                    let nr = r as i32 + dr;
                    let nc = c as i32 + dc;
                    if nr < 0 || nc < 0 || nr >= rows as i32 || nc >= cols as i32 {
                        continue;
                    }

                    if grid[nr as usize][nc as usize] == '@' {
                        neighbors += 1;
                    }
                }
            }

            if neighbors < 4 {
                accessible += 1;
            }
        }
    }

    println!("Part 1: {}", accessible);

    Ok(())
}
