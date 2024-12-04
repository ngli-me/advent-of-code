use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

const TEST_ANS: usize = 11;

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut l0: Vec<usize> = Vec::new();
        let mut l1: Vec<usize> = Vec::new();

        let _ = reader.lines().for_each(|l| {
            let binding = l.unwrap();
            //println!("{:?}", binding);
            let p: Vec<&str> = binding.split_whitespace().collect();
            assert_eq!(p.len(), 2);

            l0.push(p[0].parse().expect("Not a valid number"));
            l1.push(p[1].parse().expect("Not a valid number"));
        });

        l0.sort();
        l1.sort();

        let mut ans: usize = 0;
        l0.iter()
            .zip(l1.iter())
            .for_each(|(l0, l1)| ans += l0.abs_diff(*l1));

        Ok(ans)
    }

    assert_eq!(TEST_ANS, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut l0: HashMap<usize, usize> = HashMap::new();
        let mut l1: HashMap<usize, usize> = HashMap::new();

        let _ = reader.lines().for_each(|l| {
            let binding = l.unwrap();
            //println!("{:?}", binding);
            let p: Vec<&str> = binding.split_whitespace().collect();
            assert_eq!(p.len(), 2);

            let v0 = p[0].parse().expect("Not a valid number");
            let v1 = p[1].parse().expect("Not a valid number");
            *l0.entry(v0).or_insert(0) += 1;
            *l1.entry(v1).or_insert(0) += 1
        });

        let mut ans: usize = 0;

        for i in l0.keys() {
            match l1.get(i) {
                None => {}
                Some(j) => ans += i * l0.get(i).unwrap() * j,
            }
        }

        Ok(ans)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
