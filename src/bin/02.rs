use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use is_sorted::IsSorted;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut ans: usize = 0;

        fn check_levels(v: &Vec<usize>) -> bool {
            let mut previous = v[0] + 1;
            for i in v {
                let diff = i.abs_diff(previous);
                if diff < 1 || diff > 3 {
                    return false;
                }
                previous = *i;
            }
            true
        }

        let _ = reader.lines().for_each(|l| {
            let binding = l.unwrap();
            let l: Vec<usize> = binding
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|i| i.parse().unwrap())
                .collect();
            //println!("{:?}", l);

            if IsSorted::is_sorted(&mut l.iter()) || IsSorted::is_sorted(&mut l.iter().rev()) {
                //println!("is sorted");
                if check_levels(&l) {
                    //println!("levels safe");
                    ans += 1;
                }
            }
        });

        Ok(ans)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut ans: usize = 0;

        fn check_levels_dampener(v: &Vec<usize>) -> bool {
            if check_levels(v) {
                return true;
            }

            for i in 0..v.len() {
                let mut c = v.clone();
                c.remove(i);
                if check_levels(&c) {
                    return true;
                }
            }
            false
        }

        fn check_levels(v: &Vec<usize>) -> bool {
            let mut previous = v[0] + 1;
            for i in v {
                let diff = i.abs_diff(previous);
                if diff < 1 || diff > 3 {
                    return false;
                }
                previous = *i;
            }
            true
        }

        let _ = reader.lines().for_each(|l| {
            let binding = l.unwrap();
            let l: Vec<usize> = binding
                .split_whitespace()
                .collect::<Vec<_>>()
                .iter()
                .map(|i| i.parse().unwrap())
                .collect();
            //println!("original: {:?}", l);

            if IsSorted::is_sorted(&mut l.iter()) || IsSorted::is_sorted(&mut l.iter().rev()) {
                //println!("is sorted");
                if check_levels_dampener(&l) {
                    //println!("levels safe");
                    ans += 1;
                }
            } else {
                // Try to fix the sorting
                for i in 0..l.len() {
                    let mut c = l.clone();
                    c.remove(i);
                    //println!("{:?}", c);
                    if IsSorted::is_sorted(&mut c.iter())
                        || IsSorted::is_sorted(&mut c.iter().rev())
                    {
                        //println!("is sorted");
                        if check_levels(&c) {
                            //println!("levels safe");
                            ans += 1;
                            break;
                        }
                    }
                }
            }
        });

        Ok(ans)
    }
    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
