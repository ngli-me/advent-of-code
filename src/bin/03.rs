use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST_2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer: Vec<String> = reader.lines().flatten().collect();

        fn parse_string(s: &String) -> Result<usize> {
            let re: Regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();

            let m: Vec<usize> = re
                .find_iter(s.as_str())
                .map(|mut m| {
                    let s = &m.as_str()[4..m.len() - 1];
                    let v: Vec<&str> = s.split(',').collect();
                    //println!("{:?}", v);
                    v.iter().map(|n| n.parse::<usize>().unwrap()).product()
                })
                .collect();

            //println!("sum {:?}", m.iter().sum::<usize>());

            Ok(m.iter().sum::<usize>())
        }

        let each_line = answer
            .iter()
            .map(parse_string)
            .collect::<Result<Vec<_>>>()
            .unwrap();

        Ok(each_line.iter().sum())
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer: Vec<String> = reader.lines().flatten().collect();

        fn parse_string(s: &String) -> Result<usize> {
            let re: Regex = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
            let mut do_op: bool = true;

            let m: Vec<usize> = re.find_iter(s.as_str()).map(|mut m| {
                let m = m.as_str();
                //println!("{:?}", m);
                if m.len() == 4 {
                    // do
                    //println!("{:?}", m);
                    do_op = true;
                    0
                } else if m.chars().next() == Some('d') {
                    // dont
                    //println!("doing op");
                    do_op = false;
                    0
                } else {
                    if do_op {
                        let s = &m[4..m.len() - 1];
                        let v: Vec<&str> = s.split(',').collect();
                        //println!("{:?}", v);
                        v.iter().map(|n| n.parse::<usize>().unwrap()).product()
                    } else {
                        0
                    }
                }
            }).collect();

            //println!("sum {:?}", m.iter().sum::<usize>());

            Ok(m.iter().sum::<usize>())
        }
        parse_string(&answer.join(""))
    }
    assert_eq!(48, part2(BufReader::new(TEST_2.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
