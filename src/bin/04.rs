use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!original.is_empty());
        let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

        for original_row in original {
            for (item, transposed_row) in original_row.into_iter().zip(&mut transposed) {
                transposed_row.push(item);
            }
        }

        transposed
    }

    fn diagonalize<T: Clone>(original: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!original.is_empty());

        let mut diagonal: Vec<Vec<T>> = Vec::new();

        for j in 0..original.len() {
            let mut diag: Vec<T> = Vec::new();
            for i in 0..(original.len() - j) {
                diag.push(original[i][j + i].clone());
            }
            diagonal.push(diag);
        }

        diagonal.reverse();

        for j in 1..original.len() {
            let mut diag: Vec<T> = Vec::new();
            for i in 0..(original.len() - j) {
                diag.push(original[j + i][i].clone());
            }
            diagonal.push(diag);
        }

        diagonal
    }

    fn find_str(s: &String) -> Result<usize> {
        Ok(s.as_bytes()
            .windows(4)
            .filter(|&w| w == "XMAS".as_bytes() || w == "SAMX".as_bytes())
            .count())
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let mut lines_char: Vec<Vec<char>> =
            lines.clone().iter().map(|l| l.chars().collect()).collect();

        // transpose O(n^2) time and O(1) space algorithm
        let transpose: Vec<Vec<char>> = transpose(lines_char.clone());
        let transpose: Vec<String> = transpose.iter().map(String::from_iter).collect();

        let diagonal: Vec<Vec<char>> = diagonalize(lines_char.clone());
        let diagonal: Vec<String> = diagonal.iter().map(String::from_iter).collect();

        lines_char.reverse();
        let reverse_diagonal: Vec<Vec<char>> = diagonalize(lines_char);
        let reverse_diagonal: Vec<String> =
            reverse_diagonal.iter().map(String::from_iter).collect();

        let mut count: usize = 0;

        let _ = lines.iter().for_each(|s| {
            count += find_str(s).unwrap();
        });
        //println!("count {:?}", count);
        let _ = transpose.iter().for_each(|s| {
            count += find_str(s).unwrap();
        });
        //println!("count {:?}", count);
        let _ = diagonal.iter().for_each(|s| {
            count += find_str(s).unwrap();
        });
        //println!("count {:?}", count);
        let _ = reverse_diagonal.iter().for_each(|s| {
            count += find_str(s).unwrap();
        });
        //println!("count {:?}", count);

        //println!("rotated {:?}", transpose);

        Ok(count)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let mut lines_char: Vec<Vec<char>> =
            lines.clone().iter().map(|l| l.chars().collect()).collect();

        let mut count: usize = 0;
        for i in 1..lines_char.len() - 1 {
            for j in 1..lines_char[0].len() {
                if lines_char[i][j] == 'A' {
                    // Scan the other corners
                    if (i - 1 >= 0 && i + 1 < lines_char.len())
                        && (j - 1 >= 0 && j + 1 < lines_char[0].len())
                    {
                        let diag1: String =
                            vec![lines_char[i - 1][j - 1], lines_char[i + 1][j + 1]]
                                .iter()
                                .collect();
                        let diag2: String =
                            vec![lines_char[i - 1][j + 1], lines_char[i + 1][j - 1]]
                                .iter()
                                .collect();

                        if (diag1.eq(&"MS".to_string()) || diag1.eq(&"SM".to_string()))
                            && (diag2.eq(&"MS".to_string()) || diag2.eq(&"SM".to_string()))
                        {
                            count += 1;
                        }
                    }
                }
            }
        }

        Ok(count)
    }
    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
