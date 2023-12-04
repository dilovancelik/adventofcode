use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}
 
fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut points = 0;
    for line in lines {
        let split_line = line.split(":").collect::<Vec<&str>>();
        //let card_id = split_line[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let numbers = split_line[1].split("|").collect::<Vec<&str>>();
        let card_numbers = numbers[0].trim().split(" ").collect::<Vec<&str>>();
        let win_numbers = numbers[1].trim().split(" ").collect::<Vec<&str>>();
        let common_numbers  = card_numbers.iter().filter(|&n| win_numbers.contains(n) && n != &"").count();
        let exponent: u32 = if common_numbers == 0 { 0 } else { (common_numbers-1).try_into().unwrap() };
        if common_numbers > 0 {
            points += 1 * (2_i32).pow(exponent);
        }
    }
    points
}

fn part2(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut card_instances: HashMap<u32, i32> = HashMap::new();
    for i in 0..lines.len() {
        card_instances.insert(i as u32 + 1 as u32, 1);
    }
    let max_card_id: usize = lines.len() + 1;
    for line in lines {
        let split_line = line.split(":").collect::<Vec<&str>>();
        let card_id = split_line[0].split(" ").collect::<Vec<&str>>().last().unwrap().parse::<u32>().unwrap();
        let numbers = split_line[1].split("|").collect::<Vec<&str>>();
        let card_numbers = numbers[0].trim().split(" ").collect::<Vec<&str>>();
        let win_numbers = numbers[1].trim().split(" ").collect::<Vec<&str>>();
        let common_numbers  = card_numbers.iter().filter(|&n| win_numbers.contains(n) && n != &"").count();
        let start = if card_id + 1 <= max_card_id as u32 { card_id + 1 } else { max_card_id as u32 };
        let end = if start + common_numbers as u32 <= max_card_id as u32 { start + common_numbers  as u32 } else { max_card_id as u32 };
        let inc_value = &card_instances.get(&card_id).unwrap().clone();
        for i in start..end {
            if let Some(value) = card_instances.get_mut(&i) {
                *value += inc_value;
            }
        }
    }
    card_instances.values().sum()
}




#[test]
fn test_part1() {
    let input = include_str!("./test.txt");
    let output = part1(input);
    assert_eq!(output, 13);
}


#[test]
fn test_part2() {
    let input = include_str!("./test.txt");
    let output = part2(input);
    assert_eq!(output, 30);
}