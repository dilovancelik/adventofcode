use std::{collections::HashMap, vec};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
    let output2 = part2(input);
    dbg!(output2);
}
#[derive(Debug, PartialEq, Eq, Hash)]
enum HandRanking {
    FiveOfAkind = 6,
    FourOfAkind = 5,
    FullHouse = 4,
    ThreeOfAkind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
    Unknown = -1,
}

fn part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let char_ranking = vec![
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    let mut hands: Vec<(&str, i32, HandRanking, Vec<usize>)> = Vec::new();
    for line in lines {
        let mut cards: HashMap<char, i32> = HashMap::new();
        let row = line.split_whitespace().collect::<Vec<&str>>();
        let hand = row[0];
        let bid = row[1];
        for i in hand.chars() {
            let count = cards.entry(i).or_insert(0);
            *count += 1;
        }

        let hand_ranking: HandRanking;

        if cards.values().any(|&x| x == 5) {
            hand_ranking = HandRanking::FiveOfAkind;
        } else if cards.values().any(|&x| x == 4) {
            hand_ranking = HandRanking::FourOfAkind;
        } else if cards.values().any(|&x| x == 3) && cards.values().any(|&x| x == 2) {
            hand_ranking = HandRanking::FullHouse;
        } else if cards.values().any(|&x| x == 3) {
            hand_ranking = HandRanking::ThreeOfAkind;
        } else if cards.values().filter(|&x| *x == 2).count() == 2 {
            hand_ranking = HandRanking::TwoPair;
        } else if cards.values().any(|&x| x == 2) {
            hand_ranking = HandRanking::OnePair;
        } else if cards.values().all(|&x| x == 1) {
            hand_ranking = HandRanking::HighCard;
        } else {
            hand_ranking = HandRanking::Unknown;
        }

        let char_ranks: Vec<usize> = hand
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|&x| char_ranking.iter().position(|y| y == &x).unwrap())
            .collect();

        hands.push((hand, bid.parse::<i32>().unwrap(), hand_ranking, char_ranks));
    }

    let mut ranking_vecs: HashMap<HandRanking, Vec<(&str, i32, Vec<usize>)>> = HashMap::new();
    for hand in hands {
        ranking_vecs
            .entry(hand.2)
            .or_insert(Vec::new())
            .push((hand.0, hand.1, hand.3));
    }

    for ranking in ranking_vecs.values_mut() {
        ranking.sort_by(|a, b| a.2.cmp(&b.2));
    }

    let result_vec_vec: [Vec<(&str, i32, Vec<usize>)>; 7] = [
        ranking_vecs.get(&HandRanking::HighCard).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::OnePair).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::TwoPair).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::ThreeOfAkind).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::FullHouse).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::FourOfAkind).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::FiveOfAkind).unwrap_or(&Vec::new()).clone(),
    ];
    let result_vec = result_vec_vec.concat();

    let mut result = 0;
    for (i, hand) in result_vec.iter().enumerate() {
        result += hand.1 * (i + 1) as i32;
    }
    result as i64
}

fn part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let char_ranking = vec![
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    let mut hands: Vec<(&str, i32, HandRanking, Vec<usize>)> = Vec::new();
    for line in lines {
        let mut cards: HashMap<char, i32> = HashMap::new();
        let row = line.split_whitespace().collect::<Vec<&str>>();
        let hand = row[0];
        let bid = row[1];

        for i in hand.chars() {
            let count = cards.entry(i).or_insert(0);
            *count += 1;
        }


        let char_ranks: Vec<usize> = hand
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|&x| char_ranking.iter().position(|y| y == &x).unwrap())
            .collect();


        if cards.contains_key(&'J') {
            let mul = cards.get(&'J').unwrap().clone();
            let max_card = cards
                .iter()
                .filter(|x| x.0 != &'J' && x.1 != &5)
                .max_by_key(|x| x.1);
            match max_card {
                Some(x) => {
                    let count = cards.entry(x.0.clone()).or_insert(0);
                    *count += mul;
                    cards.remove(&'J');
                }
                None => {}
            }            
        }
        let hand_ranking: HandRanking;

        if cards.values().any(|&x| x == 5) {
            hand_ranking = HandRanking::FiveOfAkind;
        } else if cards.values().any(|&x| x == 4) {
            hand_ranking = HandRanking::FourOfAkind;
        } else if cards.values().any(|&x| x == 3) && cards.values().any(|&x| x == 2) {
            hand_ranking = HandRanking::FullHouse;
        } else if cards.values().any(|&x| x == 3) {
            hand_ranking = HandRanking::ThreeOfAkind;
        } else if cards.values().filter(|&x| *x == 2).count() == 2 {
            hand_ranking = HandRanking::TwoPair;
        } else if cards.values().any(|&x| x == 2) {
            hand_ranking = HandRanking::OnePair;
        } else if cards.values().all(|&x| x == 1) {
            hand_ranking = HandRanking::HighCard;
        } else {
            hand_ranking = HandRanking::Unknown;
        }
        
        hands.push((hand, bid.parse::<i32>().unwrap(), hand_ranking, char_ranks));
    }

    let mut ranking_vecs: HashMap<HandRanking, Vec<(&str, i32, Vec<usize>)>> = HashMap::new();
    for hand in hands {
        ranking_vecs
            .entry(hand.2)
            .or_insert(Vec::new())
            .push((hand.0, hand.1, hand.3));
    }

    for ranking in ranking_vecs.values_mut() {
        ranking.sort_by(|a, b| a.2.cmp(&b.2));
    }
    let result_vec_vec: [Vec<(&str, i32, Vec<usize>)>; 7] = [
        ranking_vecs.get(&HandRanking::HighCard).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::OnePair).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::TwoPair).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::ThreeOfAkind).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::FullHouse).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::FourOfAkind).unwrap_or(&Vec::new()).clone(),
        ranking_vecs.get(&HandRanking::FiveOfAkind).unwrap_or(&Vec::new()).clone(),
    ];
    let result_vec = result_vec_vec.concat();
    
    let mut result: i64 = 0;
    for (i, hand) in result_vec.iter().enumerate() {
        result += hand.1 as i64 * (i + 1) as i64;
    }

    result
}

#[test]
fn test_part1() {
    let input = include_str!("./test.txt");
    let output = part1(input);
    assert_eq!(output, 6440);
}

#[test]
fn test_part2() {
    let input = include_str!("./test.txt");
    let output = part2(input);
    assert_eq!(output, 5905);
}
