use std::collections::HashMap;

fn main() {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();

    let numbers = [
        ("one", '1'), ("two", '2'), ("three", '3'),
        ("four", '4'), ("five", '5'), ("six", '6'),
        ("seven", '7'), ("eight", '8'), ("nine", '9')]
        .iter().cloned().collect::<HashMap<&str, char>>();

    let mut result = 0;
    for line in lines.iter() {
        let mut numericals: Vec<char> = Vec::new();
        let mut j: usize = 0;
        for i in 0..line.len() {
            if line.chars().nth(i).unwrap().is_numeric() {
                numericals.push(line.chars().nth(i).unwrap());
                j = i;
            }
            if numbers.keys().any(|&word| line[j..i+1].contains(word)) {
                for (word, number) in &numbers {
                    if line[j..i+1].contains(word) {
                        numericals.push(*number);
                        
                    }
                }
                j = i;
            } 
        }
        let first_number_char = numericals[0];
        let last_number_char = numericals[numericals.len() - 1];
        result += format!("{}{}", first_number_char, last_number_char).parse::<u32>().unwrap();
    }
    result
}