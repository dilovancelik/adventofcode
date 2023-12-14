
fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}
 
fn part1(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let times = lines[0]
        .split(":").collect::<Vec<&str>>()[1]
        .split_whitespace().collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let distances = lines[1]
        .split(":").collect::<Vec<&str>>()[1]
        .split_whitespace().collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

        
    let mut combinations: Vec<i32> = Vec::new();
    for (i, race_time) in times.iter().enumerate() {
        let distance = distances[i];
        let mut possible_combinations = 0;
        for j in 1..*race_time {
            //dbg!(i, j, race_time, distance, j * (race_time - j));
            if j * (race_time - j) > distance {
                possible_combinations += 1;
            }
        }
        combinations.push(possible_combinations);
    }
    combinations.iter().fold(1, |acc, x| acc * x)
}


fn part2(input: &str) -> usize {
    
    let lines: Vec<&str> = input.lines().collect();
    let time = lines[0]
        .split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter(|x| x != &' ')
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>()
        .parse::<i64>().unwrap();

    let distance = lines[1]
        .split(":").collect::<Vec<&str>>()[1]
        .chars()
        .filter(|x| x != &' ')
        .collect::<Vec<char>>()
        .iter()
        .collect::<String>()
        .parse::<i64>().unwrap();

    let mut possible_combinations = 0;
    for j in 1..time {
        //dbg!(i, j, race_time, distance, j * (race_time - j));
        if j * (time - j) > distance {
            possible_combinations += 1;
        }
    }
    possible_combinations
}


#[test]
fn test_part1() {
    let input = include_str!("./test.txt");
    let output = part1(input);
    assert_eq!(output, 288);
}


#[test]
fn test_part2() {
    let input = include_str!("./test.txt");
    let output = part2(input);
    assert_eq!(output, 71503);
}