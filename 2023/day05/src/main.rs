
use std::collections::HashMap;
// TODO fix performance in hashmap
#[derive(Debug)]
struct Mapping {
    destination: String,
    map: Vec<(i64, i64, i64)>,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> i64 {
    let mut mappings: Vec<Vec<&str>> = Vec::new();
    mappings.push(Vec::new());
    let mut i = 0;

    for line in input.lines() {

        if line == "" {
            i += 1;
            mappings.push(Vec::new());
            continue;
        }
        
        mappings[i].push(line);
    }


    let seeds = mappings[0][0]
        .split(":").collect::<Vec<&str>>()[1].trim()
        .split(" ").collect::<Vec<&str>>()
        .iter()
        .map(|&n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut mapping_table: HashMap<&str, Mapping> = HashMap::new();

    for mapping in &mappings[1..] {
        let map_name = mapping[0].split(" ").collect::<Vec<&str>>()[0].split("-to-").collect::<Vec<&str>>();
        let source_name = map_name[0];
        let destination_name = map_name[1];
        mapping_table.insert(
            source_name, Mapping {
            destination: destination_name.to_string(),
            map: Vec::new(),
        });
        
        
        for map_value in &mapping[1..] {
            let maps = map_value
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let destination = maps[0];
            let source = maps[1];
            let range = maps[2];
            mapping_table.get_mut(source_name).unwrap().map.push((source, destination, range));
        }
    }

    let mut seed_location: Vec<i64> = Vec::new();
    for seed in &seeds {
        let mut source = "seed";
        let mut value = seed.clone();
        
        while source != "location" {
            let mut sorted_map = mapping_table.get(source).unwrap().map.clone();
            sorted_map.sort_by(|a, b| a.0.cmp(&b.0));
            for map in sorted_map.into_iter() {
                if map.0 <= value && value <= map.0 + map.2 {
                    let distance = value - map.0;
                    value = map.1 + distance;
                    break;
                }
            }
            source = mapping_table.get(source).unwrap().destination.as_str();
        }
        
        seed_location.push(value);
    }
    seed_location.iter().min().unwrap().clone()
}

fn part2(input: &str) -> i64 {
    
    let mut mappings: Vec<Vec<&str>> = Vec::new();
    mappings.push(Vec::new());
    let mut i = 0;

    for line in input.lines() {

        if line == "" {
            i += 1;
            mappings.push(Vec::new());
            continue;
        }
        
        mappings[i].push(line);
    }


    let seeds = mappings[0][0]
        .split(":").collect::<Vec<&str>>()[1].trim()
        .split(" ").collect::<Vec<&str>>()
        .iter()
        .map(|&n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    
    let mut seed_i = 0;
    let mut seed_vec: Vec<(i64, i64)> = Vec::new();   
    while seed_i <= seeds.len() - 2 {
        seed_vec.push((seeds[seed_i], seeds[seed_i + 1]));
        seed_i += 2;
    }

    let mut mapping_table: HashMap<&str, Mapping> = HashMap::new();

    for mapping in &mappings[1..] {
        let map_name = mapping[0].split(" ").collect::<Vec<&str>>()[0].split("-to-").collect::<Vec<&str>>();
        let source_name = map_name[0];
        let destination_name = map_name[1];
        mapping_table.insert(
            source_name, Mapping {
            destination: destination_name.to_string(),
            map: Vec::new(),
        });
        
        
        for map_value in &mapping[1..] {
            let maps = map_value
                .trim()
                .split(" ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|&n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();
            let destination = maps[0];
            let source = maps[1];
            let range = maps[2];
            mapping_table.get_mut(source_name).unwrap().map.push((source, destination, range));
        }
        mapping_table.get_mut(source_name).unwrap().map.sort_by(|a, b| a.0.cmp(&b.0));

        let first_map = mapping_table.get(source_name).unwrap().map[0].0;
        let last_map = mapping_table.get(source_name).unwrap().map.last().unwrap().clone();
        if first_map != 0 {
            mapping_table.get_mut(source_name).unwrap().map.push((0, 0, first_map));
        }
        mapping_table.get_mut(source_name).unwrap().map.push((last_map.0 + last_map.2, last_map.0 + last_map.2, i64::MAX - (last_map.0 + last_map.2)));

        mapping_table.get_mut(source_name).unwrap().map.sort_by(|a, b| a.0.cmp(&b.0));

        let mut temp_map: Vec<(i64, i64, i64)> = Vec::new();
        for (i, map) in mapping_table.get(source_name).unwrap().map.iter().enumerate() {
            if i == mapping_table.get(source_name).unwrap().map.len() - 1 {
                break;
            }
            let next_map = mapping_table.get(source_name).unwrap().map.get(i + 1).unwrap().0;
            let expected_next_map = map.0 + map.2 - 1;
            if next_map != expected_next_map {
                temp_map.push((expected_next_map, expected_next_map, next_map - expected_next_map));
            }
        }
        for map in temp_map {
            mapping_table.get_mut(source_name).unwrap().map.push(map);
        }
        
    }

    let mut source = "seed";
    let mut range: Vec<(i64, i64)> = seed_vec.clone();
    while source != "location" {
        let mut temp_range: Vec<(i64, i64)> = Vec::new();
        let mut sorted_map = mapping_table.get(source).unwrap().map.clone();
        sorted_map.sort_by(|a, b| a.0.cmp(&b.0));
        for value in range.iter() {
            let mut start = value.0;
            let mut range = value.1;
            while range > 0 {
                for map in sorted_map.iter() {
                    if map.0 <= start && start < map.0 + map.2 {
                        if range <= map.2 - (start - map.0) {
                            temp_range.push((map.1 + (start - map.0), range));
                            range = 0;
                        } else {
                            temp_range.push((map.1 + (start - map.0), map.2 - (start - map.0)));
                            range -= map.2 - (start - map.0);
                            start += map.2 - (start - map.0);
                        }
                    }
                }
            }
        }
        range = temp_range;
        source = mapping_table.get(source).unwrap().destination.as_str();
    }
    range.sort_by(|a, b| a.0.cmp(&b.0));
    range[0].0
}

#[test]
fn test_part1() {
    let input = include_str!("./test.txt");
    let output = part1(input);
    assert_eq!(output, 35);
}


#[test]
fn test_part2() {
    let input = include_str!("./test.txt");
    let output = part2(input);
    assert_eq!(output, 46);
}