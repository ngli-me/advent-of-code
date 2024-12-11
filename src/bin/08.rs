use std::collections::HashMap;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        let mut y: usize = 0;
        let mut x: usize = 0;
        lines.iter().for_each(|line| {
            line.chars().for_each(|c| {
                if c != '.' {
                    antennas.entry(c).or_default().push((x, y));
                }
                x += 1;
            });
            y += 1;
            x = 0;
        });

        let mut antinodes: Vec<(usize, usize)> = Vec::new();

        for channel in antennas.keys() {
            let points = antennas.get(channel).unwrap();

            let mut x1: usize = 0;

            while x1 < points.len() {
                let mut x2: usize = x1 + 1;

                while x2 < points.len() {
                    // Calculate the antinodes for points x1, x2
                    let p1 = antennas.get(channel).unwrap()[x1];
                    let p2 = antennas.get(channel).unwrap()[x2];

                    let x_diff = p1.1.abs_diff(p2.1);
                    let y_diff = p1.0.abs_diff(p2.0);

                    // Determine the orientation of the points
                    match p1.1 > p2.1 {
                        true => {

                        }
                        false => {

                        }
                    }


                    x2 += 1;
                }

                x1 += 1;
            }
        }


        Ok(0)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    /*println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        Ok(0)
    }
    assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);*/
    //endregion

    Ok(())
}
