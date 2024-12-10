use crate::Dir::North;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{any, iter};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

#[derive(Clone, Copy, Eq, PartialEq)]
enum Dir {
    North,
    East,
    South,
    West,
}

enum State {
    Good,
    Blocked,
    Edge,
}

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn get_position(grid: &Vec<Vec<char>>) -> (usize, usize) {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == 'X' {
                    return (y, x);
                }
            }
        }
        // Should be an error state
        panic!("No initial position found")
    }

    fn get_unique_moves(grid: &Vec<Vec<char>>) -> usize {
        let mut total: usize = 0;
        grid.iter().for_each(|row| {
            row.iter().for_each(|p| {
                if *p == 'X' {
                    total += 1;
                }
            })
        });
        total
    }

    fn check_position(grid: &Vec<Vec<char>>, mut pos: (usize, usize), d: Dir) -> State {
        match d {
            Dir::North => if pos.0 == 0 {
                return State::Edge;
            },
            Dir::East => if pos.1 == grid.len() - 1 {
                return State::Edge;
            },
            Dir::South => if pos.0 == grid.len() - 1 {
                return State::Edge;
            },
            Dir::West => if pos.1 == 0 {
                return State::Edge;
            },
        }
        match d {
            Dir::North => pos.0 -= 1,
            Dir::East => pos.1 += 1,
            Dir::South => pos.0 += 1,
            Dir::West => pos.1 -= 1,
        }
        match grid[pos.0][pos.1] {
            '#' => State::Blocked,
            '.' | 'X' => State::Good,
            _ => panic!("Unexpected position state"),
        }
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        // initialize grid
        // dir will be N,E,S,W
        let mut grid: Vec<Vec<char>> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        if c == '^' {
                            return 'X';
                        }
                        c
                    })
                    .collect()
            })
            .collect();

        let mut p: (usize, usize) = get_position(&grid);
        let mut d: Dir = Dir::North;

        loop {
            match check_position(&grid, p, d) {
                State::Good => {
                    match d {
                        Dir::North => p.0 -= 1,
                        Dir::East => p.1 += 1,
                        Dir::South => p.0 += 1,
                        Dir::West => p.1 -= 1,
                    }
                    grid[p.0][p.1] = 'X';
                }
                State::Blocked => match d {
                    Dir::North => d = Dir::East,
                    Dir::East => d = Dir::South,
                    Dir::South => d = Dir::West,
                    Dir::West => d = Dir::North,
                },
                State::Edge => {
                    break;
                }
            }
        }

        for l in grid.iter() {
            println!("{:?}", l);
        }

        Ok(get_unique_moves(&grid))
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn new_check_position(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> State {
        if (pos.0 == 0 || pos.0 >= grid.len()) || (pos.1 == 0 || pos.1 >= grid[0].len()) {
            return State::Edge;
        }
        match grid[pos.0][pos.1] {
            '#' => State::Blocked,
            '.' | 'X' => State::Good,
            _ => panic!("Unexpected position state"),
        }
    }

    fn is_loop(mut p: (usize, usize), mut d: Dir, mut grid: &mut Vec<Vec<char>>) -> bool {
        let mut visited: Vec<(usize, usize, Dir)> = Vec::new();

        let mut looping: bool = false;
        loop {
            match check_position(&grid, p, d) {
                State::Good => {
                    match d {
                        Dir::North => p.0 -= 1,
                        Dir::East => p.1 += 1,
                        Dir::South => p.0 += 1,
                        Dir::West => p.1 -= 1,
                    }
                    grid[p.0][p.1] = 'X';
                }
                State::Blocked => {
                    if visited.iter().any(|x| x.0 == p.0 && x.1 == p.1 && x.2 == d) {
                        return true
                    }
                    visited.push((p.0, p.1, d));
                    match d {
                        Dir::North => d = Dir::East,
                        Dir::East => d = Dir::South,
                        Dir::South => d = Dir::West,
                        Dir::West => d = Dir::North,
                    }
                },
                State::Edge => {
                    break;
                }
            }
        }

        looping
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

        // initialize grid
        // dir will be N,E,S,W
        let mut grid: Vec<Vec<char>> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| {
                        if c == '^' {
                            return 'X';
                        }
                        c
                    })
                    .collect()
            })
            .collect();

        let mut p: (usize, usize) = get_position(&grid);
        let mut d: Dir = Dir::North;

        let mut count: usize = 0;

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                // If it doesn't have a barrier or is the starting location
                if grid[y][x] != 'X' && grid[y][x] != '#' {
                    let mut modified = grid.clone();
                    modified[y][x] = '#';
                    if is_loop(p.clone(), d.clone(), &mut modified) {
                        count += 1;
                    }
                }
            }
        }

        /*for l in grid.iter() {
            println!("{:?}", l);
        }*/

        Ok(count)
    }
    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
