use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn validate_update(page_ordering_rules: &HashMap<usize, Vec<usize>>, u: &Vec<usize>) -> bool {
        let mut seen: HashMap<usize, usize> = HashMap::new();
        let mut status: bool = true;

        for page in u {
            // if it's already been seen or isnt in the rules, then we can skip, otherwise fvalidate
            if !seen.keys().contains(&page) && page_ordering_rules.contains_key(&page) {
                // validate conditions
                for r in page_ordering_rules.get(page).unwrap().iter() {
                    if !seen.keys().contains(r) {
                        if u.contains(r) {
                            // Double check if its in the list at all
                            status = false;
                            break;
                        }
                    }
                }
                if !status {
                    break;
                }
            }
            seen.entry(*page).and_modify(|mut c| *c += 1).or_insert(1);
        }

        status
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut page_ordering_rules: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut update: Vec<Vec<usize>> = Vec::new();
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        let mut total: usize = 0;

        lines.iter().for_each(|l| {
            // looks like its always 5 chars

            if l.contains('|') {
                let rule: Vec<&str> = l.split('|').collect();
                assert_eq!(rule.len(), 2);
                let before: usize = rule[0].parse::<usize>().unwrap();
                page_ordering_rules
                    .entry(rule[1].parse::<usize>().unwrap())
                    .and_modify(|s| s.push(before))
                    .or_insert(vec![before]);
            } else if l.contains(',') {
                let u: Vec<&str> = l.split(',').collect();
                let u = u
                    .iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                update.push(u);
            }
        });

        //println!("{:?}", page_ordering_rules);

        update.iter().for_each(|u| {
            //println!("update: {:?}", u);
            let status: bool = validate_update(&page_ordering_rules, u);
            //println!("status: {:?}", status);
            if status {
                //println!("adding: {:?}", u.get(u.len() / 2).unwrap());
                total += u.get(u.len() / 2).unwrap();
            }
        });

        //println!("{:?}", update);

        Ok(total)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut page_ordering_rules: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut update: Vec<Vec<usize>> = Vec::new();
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        let mut total: usize = 0;

        lines.iter().for_each(|l| {
            // looks like its always 5 chars
            if l.contains('|') {
                let rule: Vec<&str> = l.split('|').collect();
                assert_eq!(rule.len(), 2);
                let before: usize = rule[0].parse::<usize>().unwrap();
                page_ordering_rules
                    .entry(rule[1].parse::<usize>().unwrap())
                    .and_modify(|s| s.push(before))
                    .or_insert(vec![before]);
            } else if l.contains(',') {
                let u: Vec<&str> = l.split(',').collect();
                let u = u
                    .iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
                update.push(u);
            }
        });

        let mut incorrect: Vec<Vec<usize>> = Vec::new();

        update.iter().for_each(|u| {
            let status: bool = validate_update(&page_ordering_rules, u);
            if !status {
                // Ones that failed
                incorrect.push(u.clone());
                //total += u.get(u.len() / 2).unwrap();
            }
        });

        incorrect.iter_mut().for_each(|mut u| {
            u.sort_by(|a, b| {
                if page_ordering_rules.contains_key(a) && page_ordering_rules[a].contains(b) {
                    return std::cmp::Ordering::Greater;
                } else if page_ordering_rules.contains_key(b) && page_ordering_rules[b].contains(a) {
                    return std::cmp::Ordering::Less;
                } else {
                    return std::cmp::Ordering::Equal;
                }
            })
        });


        incorrect.iter().for_each(|u| total += u.get(u.len() / 2).unwrap());

        Ok(total)
    }
    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
