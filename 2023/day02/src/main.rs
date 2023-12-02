use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}

fn part1(input: &str) -> u32 {


    let colours = [("red", 12), ("green", 13), ("blue", 14)]
        .iter().cloned().collect::<HashMap<&str, u32>>();

    let lines: Vec<&str> = input.lines().collect();

    let mut posible_games: Vec<u32> = Vec::new();

    for line in lines.iter() {
        let split_line = line.split(":").collect::<Vec<&str>>();
        let game_id = split_line[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let games = split_line[1].split(";").collect::<Vec<&str>>();

        let mut reds: Vec<u32> = Vec::new();
        let mut greens: Vec<u32> = Vec::new();
        let mut blues: Vec<u32> = Vec::new();
        for game in games {

            let turns = game.split(",").collect::<Vec<&str>>();
            
            for turn in turns {
                let draw = turn.split(" ").collect::<Vec<&str>>();

                let number = draw[1];
                let colour = draw[2];
                match colour {
                    "red" => reds.push(number.parse::<u32>().unwrap()),
                    "green" => greens.push(number.parse::<u32>().unwrap()),
                    "blue" => blues.push(number.parse::<u32>().unwrap()),
                    _ => println!("Error"),
                }
            }
        } 
        if reds.iter().max().unwrap_or(&0) <= colours.get("red").unwrap() && 
            greens.iter().max().unwrap_or(&0) <= colours.get("green").unwrap() && 
            blues.iter().max().unwrap_or(&0) <= colours.get("blue").unwrap() {
            posible_games.push(game_id);
        }

    }
    let result = posible_games.iter().sum();
    result
}

fn part2(input: &str) -> u32 {
    
    let colours = [("red", 12), ("green", 13), ("blue", 14)]
        .iter().cloned().collect::<HashMap<&str, u32>>();

    let lines: Vec<&str> = input.lines().collect();

    let mut list_power: Vec<u32> = Vec::new();

    for line in lines.iter() {
        let split_line = line.split(":").collect::<Vec<&str>>();
        let game_id = split_line[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let games = split_line[1].split(";").collect::<Vec<&str>>();

        let mut reds: Vec<u32> = Vec::new();
        let mut greens: Vec<u32> = Vec::new();
        let mut blues: Vec<u32> = Vec::new();
        for game in games {

            let turns = game.split(",").collect::<Vec<&str>>();
            
            for turn in turns {
                let draw = turn.split(" ").collect::<Vec<&str>>();

                let number = draw[1];
                let colour = draw[2];
                match colour {
                    "red" => reds.push(number.parse::<u32>().unwrap()),
                    "green" => greens.push(number.parse::<u32>().unwrap()),
                    "blue" => blues.push(number.parse::<u32>().unwrap()),
                    _ => println!("Error"),
                }
            }
        } 
        let min_reds = reds.iter().max().unwrap_or(&0);
        let min_greens = greens.iter().max().unwrap_or(&0);
        let min_blues = blues.iter().max().unwrap_or(&0);
        let power = min_reds * min_greens * min_blues;
        list_power.push(power);
        

    }
    let result = list_power.iter().sum();
    result
}

