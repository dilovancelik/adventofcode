use std::{collections::HashMap, vec};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let rl_line = lines[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    let mut mappings: HashMap<&str, Vec<String>> = HashMap::new();
    let maps = lines[2..]
        .iter()
        .map(|line| {
            let map_line = line.split("=").collect::<Vec<&str>>();
            let map_key = map_line[0].trim();
            let map_value = map_line[1]
                .trim()
                .split(", ")
                .map(|x| if x.contains('(') { 
                                    x.replace("(", "") 
                                } else { 
                                    x.replace(")", "")})
                .collect::<Vec<String>>();
            return (map_key, (map_value))
        })
        .collect::<Vec<(&str, Vec<String>)>>();
    maps.iter().for_each(|(k, v)| {
        mappings.insert(k, v.to_vec());
    });
    let mut loc = "AAA";
    let mut steps: i32 = 0;
    let mut rl_index = 0;

    while loc != "ZZZ" {
        steps += 1;
        if rl_index >= rl_line.len() { 
            rl_index = 0;
        };
        let map = mappings.get(loc).unwrap();
        loc = map[rl_line[rl_index]].as_str();
        rl_index += 1;
    };
    steps as i64

}

fn part2(input: &str) -> i64 {
    
    let lines: Vec<&str> = input.lines().collect();
    let rl_line = lines[0]
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();
    let mut mappings: HashMap<&str, Vec<String>> = HashMap::new();
    let maps = lines[2..]
        .iter()
        .map(|line| {
            let map_line = line.split("=").collect::<Vec<&str>>();
            let map_key = map_line[0].trim();
            let map_value = map_line[1]
                .trim()
                .split(", ")
                .map(|x| if x.contains('(') { 
                                    x.replace("(", "") 
                                } else { 
                                    x.replace(")", "")})
                .collect::<Vec<String>>();
            return (map_key, (map_value))
        })
        .collect::<Vec<(&str, Vec<String>)>>();
    maps.iter().for_each(|(k, v)| {
        mappings.insert(k, v.to_vec());
    });
    let mut keys = mappings.keys().cloned().filter(|x| x.ends_with("A")).collect::<Vec<&str>>();
    let mut step_vec: Vec<i64> = vec![];
    for key in keys.iter() {
        let mut loc = key.clone();
        let mut steps: i32 = 0;
        let mut rl_index = 0;
        while !loc.ends_with("Z") {
            steps += 1;
            if rl_index >= rl_line.len() { 
                rl_index = 0;
            };
            let map = mappings.get(key.clone()).unwrap();
            loc = map[rl_line[rl_index]].as_str();
            rl_index += 1;
        };
        dbg!(&loc);
        step_vec.push(steps as i64);
    }
    step_vec[0] as i64
}

#[test]
fn test_part1_1() {
    let input = include_str!("./test1.txt");
    let output = part1(input);
    assert_eq!(output, 2);
}


#[test]
fn test_part1_2() {
    let input = include_str!("./test2.txt");
    let output = part1(input);
    assert_eq!(output, 6);
}

#[test]
fn test_part2() {
    let input = include_str!("./test2.txt");
    let output = part2(input);
    assert_eq!(output, 6);
}

