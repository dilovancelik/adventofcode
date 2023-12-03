fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Mapping {
    row: usize,
    symbols: Vec<usize>,
    numerics: Vec<Vec<usize>>,
}

fn part1(input: &str) -> i32 {
    let (matrix, mappings) = generate_matrix(input);
    let summable_numbers_index = check_summable_numbers(mappings);
    let summable_numbers = extract_summable_numbers(summable_numbers_index, matrix);
    summable_numbers.iter().sum()

}

fn generate_matrix(input: &str) -> (Vec<Vec<char>>, Vec<Mapping>) {
    let lines: Vec<&str> = input.lines().collect();
    let mut mappings: Vec<Mapping> = Vec::new();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut row_number: usize = 0;
    for line in lines {
        let mut row: Vec<u32> = Vec::new();
        let mut row = line.chars().collect::<Vec<char>>();
        let numerics: Vec<usize> = row.iter().enumerate()
            .filter_map(|(i, c)| if c.is_numeric() { Some(i) } else { None })
            .collect();
        let symbols: Vec<usize> = row.iter().enumerate()
            .filter_map(|(i, c)| if !c.is_numeric() && *c != '.' { Some(i) } else { None })
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


#[test]
fn test_part1() {
    let input = include_str!("./test.txt");
    let output = part1(input);
    assert_eq!(output, 4361);
}