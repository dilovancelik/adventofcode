use core::num;
use std::slice::SplitInclusiveMut;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}

#[derive(Debug)]
struct Mapping {
    row: usize,
    symbols: Vec<usize>,
    numerics: Vec<Vec<usize>>,
    gears: Vec<usize>,
}

fn part1(input: &str) -> i32 {
    let (matrix, mappings) = generate_matrix(input);
    let summable_numbers_index = check_summable_numbers(mappings);
    let summable_numbers = extract_summable_numbers(summable_numbers_index, matrix);
    summable_numbers.iter().sum()
}

fn part2(input: &str) -> i32 {
    let (matrix, mappings) = generate_matrix(input);
    let summable_numbers_index = retrieve_gear_numbers(mappings);    
    let summable_numbers = extract_summable_gear_numbers(summable_numbers_index, matrix);
    //dbg!(&summable_numbers);
    summable_numbers.iter().sum()
}

fn generate_matrix(input: &str) -> (Vec<Vec<char>>, Vec<Mapping>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut row_number: usize = 0;
    for line in lines {
        let row = line.chars().collect::<Vec<char>>();
        let numerics: Vec<usize> = row.iter().enumerate()
            .filter_map(|(i, c)| if c.is_numeric() { Some(i) } else { None })
            .collect();
        let symbols: Vec<usize> = row.iter().enumerate()
            .filter_map(|(i, c)| if !c.is_numeric() && *c != '.' { Some(i) } else { None })
            .collect();
        let gears: Vec<usize> = row.iter().enumerate()
            .filter_map(|(i, c)| if *c == '*' { Some(i) } else { None })
            .collect();

        let mut split_index: Vec<usize> = Vec::new();
        for (i, c) in numerics.iter().enumerate() {
            if i == 0 {
                continue;
            } else {
                let prev = numerics[i - 1];
                let diff = c - prev;
                if diff > 1 {
                    split_index.push(i);
                }               
            }
        }
        let mut split_numerics: Vec<Vec<usize>> = Vec::new();
        let mut start = 0;
        if split_index.len() == 0 {
            split_numerics.push(numerics.to_vec());
        } else {
            for(i, value) in split_index.iter().enumerate()  {
                if i == split_index.len() - 1 {
                    split_numerics.push(numerics[start..*value].to_vec());
                    split_numerics.push(numerics[*value..].to_vec());
                } else {
                    split_numerics.push(numerics[start..*value].to_vec());
                }
                start = *value;
            }
        }

        matrix.push(row);
        mappings.push(Mapping {
            row: row_number,
            symbols: symbols,
            numerics: split_numerics,
            gears: gears,
        });
        row_number += 1;
    }
    (matrix, mappings)
}

fn check_summable_numbers(mappings: Vec<Mapping>) -> Vec<Vec<Vec<usize>>> {
    let mut summable_numbers_index: Vec<Vec<Vec<usize>>> = Vec::new();
    for mapping in &mappings {
        let mut summable: Vec<Vec<usize>> = Vec::new();
        for numeric in mapping.numerics.iter() {
            if numeric.len() == 0 {
                continue;
            }
            let start = if numeric[0] > 0 { numeric[0] - 1 } else { 0 };
            let end = numeric[numeric.len() - 1] + 1;
            if mapping.symbols.iter().any(|&x| x >= start && x <= end) {
                summable.push(numeric.to_vec());
                continue;
            } 
            if mapping.row > 0 {
                if mappings[mapping.row - 1].symbols.iter().any(|&x| x >= start && x <= end) {
                    summable.push(numeric.to_vec());
                    continue;
                }
            }
            if mapping.row < mappings.len() - 1 {
                if mappings[mapping.row + 1].symbols.iter().any(|&x| x >= start && x <= end) {
                    summable.push(numeric.to_vec());
                    continue;
                }
            }
        }
        summable_numbers_index.push(summable);
    }
    summable_numbers_index
}

fn extract_summable_numbers(summable_numbers_index: Vec<Vec<Vec<usize>>>, matrix: Vec<Vec<char>>) -> Vec<i32> {
    let mut summable_numbers: Vec<i32> = Vec::new();
    for (row, numerics) in summable_numbers_index.iter().enumerate() {
        for numeric in numerics {
            let start = numeric[0];
            let end = numeric[numeric.len() - 1];
            let sum_num_str: String = matrix[row][start..end+1].iter().collect();
            let sum_num: i32 = sum_num_str.parse::<i32>().unwrap();
            summable_numbers.push(sum_num);
        }
    }
    summable_numbers
}


fn retrieve_gear_numbers(mappings: Vec<Mapping>) -> Vec<Vec<Vec<usize>>> {
    let mut gear_numbers: Vec<Vec<Vec<usize>>> = Vec::new();

    for mapping in &mappings {
        for gear in &mapping.gears {
            let start = if gear > &0 { gear - 1 } else { 0 };
            let end = gear + 1;
            let mut adjacent_numerics: Vec<Vec<usize>> = Vec::new();
            for numeric in mapping.numerics.iter() {
                
                if numeric.len() == 0 {
                    continue;
                }
                let mut map: Vec<usize> = numeric.to_vec();
                let gear_range = start..end;
                let numeric_range = numeric[0]..numeric[numeric.len() - 1];
                if gear_range.start <= numeric_range.end && numeric_range.start <= gear_range.end {
                    map.insert(0, mapping.row);
                    adjacent_numerics.push(map);
                }
            }
            if mapping.row > 0 {
                for numeric in mappings[mapping.row - 1].numerics.iter() {
                    if numeric.len() == 0 {
                        continue;
                    }
                    let mut map: Vec<usize> = numeric.to_vec();
                    let gear_range = start..end;
                    let numeric_range = numeric[0]..numeric[numeric.len() - 1];
                    if gear_range.start <= numeric_range.end && numeric_range.start <= gear_range.end {
                        map.insert(0, mapping.row - 1);
                        adjacent_numerics.push(map);
                    }
                }
            }
            if mapping.row < mappings.len() - 1 {
                for numeric in mappings[mapping.row + 1].numerics.iter() {
                    if numeric.len() == 0 {
                        continue;
                    }
                    let mut map: Vec<usize> = numeric.to_vec();
                    let gear_range = start..end;
                    let numeric_range = numeric[0]..numeric[numeric.len() - 1];
                    if gear_range.start <= numeric_range.end && numeric_range.start <= gear_range.end {
                        map.insert(0, mapping.row + 1);
                        adjacent_numerics.push(map);
                    }
                }
            }
            if adjacent_numerics.len() == 2 {
                gear_numbers.push(adjacent_numerics);
            }
        }
    }
    gear_numbers
}


fn extract_summable_gear_numbers(summable_numbers_index: Vec<Vec<Vec<usize>>>, matrix: Vec<Vec<char>>) -> Vec<i32> {
    let mut summable_numbers: Vec<Vec<i32>> = Vec::new();
    for gear_row in summable_numbers_index.iter() {
        let mut summable_row: Vec<i32> = Vec::new();
        for index in gear_row {
            let row = index[0];
            let start = index[1];
            let end = index[index.len() - 1];
            let sum_num_str: String = matrix[row][start..end+1].iter().collect();
            let sum_num: i32 = sum_num_str.parse::<i32>().unwrap();
            summable_row.push(sum_num);
        }
        summable_numbers.push(summable_row);
    }
    let mut product_vec: Vec<i32> = Vec::new();
    for gear in summable_numbers.iter() {
        let product: i32 = gear.iter().product();
        product_vec.push(product);
    }
    product_vec
}

#[test]
fn test_part1() {
    let input = include_str!("./test.txt");
    let output = part1(input);
    assert_eq!(output, 4361);
}


#[test]
fn test_part2() {
    let input = include_str!("./test.txt");
    let output = part2(input);
    assert_eq!(output, 467835);
}