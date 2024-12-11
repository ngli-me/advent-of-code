use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn is_reachable(target: usize, nums: &[usize]) -> bool {
        if nums.len() == 1 {
            return target == nums[0]
        }
        let (&last, rest) = nums.split_last().unwrap();
        if target % last == 0 && is_reachable(target / last, rest) {
            return true;
        }
        if target > last && is_reachable(target - last, rest) {
            return true;
        }
        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut eqs: Vec<(usize, Vec<usize>)> = Vec::new();

        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        lines.iter().for_each(|line| {
            let split: Vec<&str> = line.split(':').collect();
            assert_eq!(split.len(), 2);
            eqs.push((
                split[0].parse::<usize>().unwrap(),
                split[1]
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            ));
        });

        let total: usize = eqs
            .iter()
            .filter(|(r, n)| is_reachable(*r, n))
            .map(|(r, n)| r)
            .sum();

        Ok(total)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn is_reachable_concat(target: usize, nums: &[usize]) -> bool {
        if nums.len() == 1 {
            return target == nums[0]
        }
        let (&last, rest) = nums.split_last().unwrap();
        if target % last == 0 && is_reachable_concat(target / last, rest) {
            return true;
        }
        if target > last && is_reachable_concat(target - last, rest) {
            return true;
        }
        let last_len = last.ilog10() + 1;
        let magnitude = 10usize.pow(last_len);
        let target_len = target.ilog10() + 1;
        let ending = target % magnitude;
        // last == ending compares if `last` is the final digit(s) of target
        if target_len > last_len && last == ending && is_reachable_concat(target / magnitude, rest) {
            return true;
        }

        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut eqs: Vec<(usize, Vec<usize>)> = Vec::new();

        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        lines.iter().for_each(|line| {
            let split: Vec<&str> = line.split(':').collect();
            assert_eq!(split.len(), 2);
            eqs.push((
                split[0].parse::<usize>().unwrap(),
                split[1]
                    .trim()
                    .split(' ')
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            ));
        });

        let total: usize = eqs
            .iter()
            .filter(|(r, n)| is_reachable_concat(*r, n))
            .map(|(r, n)| r)
            .sum();

        Ok(total)
    }
    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
