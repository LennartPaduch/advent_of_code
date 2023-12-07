use std::collections::HashMap;

#[derive(Debug)]
struct Hand {
    cards: Vec<usize>,
    hand_strength: usize,
    bid: u64,
}
// 253866470
//254494947
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let hands: Vec<&str> = input.lines().collect();

    // Part 1:
    let mut parsed_hands: Vec<Hand> = parse_hands(&hands, false);
    sort_hands(&mut parsed_hands);
    let mut sum = get_total_winnings(parsed_hands);
    println!("Part 1: {}", sum);

    // Part 2:
    parsed_hands = parse_hands(&hands, true);
    sort_hands(&mut parsed_hands);
    sum = get_total_winnings(parsed_hands);
    println!("Part 2: {}", sum);
}

fn get_total_winnings(hands: Vec<Hand>) -> u64 {
    let total_winnings = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid * (i + 1) as u64));
    total_winnings
}

fn parse_hands(hands: &Vec<&str>, is_joker_enabled: bool) -> Vec<Hand> {
    hands
        .iter()
        .map(|hand| {
            let hand_parts: Vec<&str> = hand.split(" ").collect();
            let bid: u64 = hand_parts[1].parse().unwrap();
            let (hand_strength, hand) = determine_hand_strength(hand_parts[0], is_joker_enabled);
            Hand {
                cards: hand,
                hand_strength,
                bid,
            }
        })
        .collect()
}

fn sort_hands(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| match a.hand_strength.cmp(&b.hand_strength) {
        std::cmp::Ordering::Equal => {
            for (card_a, card_b) in a.cards.iter().zip(&b.cards) {
                match card_a.cmp(card_b) {
                    std::cmp::Ordering::Equal => continue,
                    non_equal => return non_equal,
                }
            }
            std::cmp::Ordering::Equal
        }
        other => other,
    });
}

fn translate_card_values(cards: &str, is_joker_enabled: bool) -> Vec<usize> {
    let card_val_map: HashMap<char, usize> = HashMap::from([
        ('J', if is_joker_enabled { 0 } else { 10 }),
        ('2', 1),
        ('3', 2),
        ('4', 3),
        ('5', 4),
        ('6', 5),
        ('7', 6),
        ('8', 7),
        ('9', 8),
        ('T', 9),
        ('Q', 11),
        ('K', 12),
        ('A', 13),
    ]);
    let translated_hands: Vec<usize> = cards
        .chars()
        .filter_map(|c: char| card_val_map.get(&c))
        .copied()
        .collect();
    translated_hands
}

fn determine_hand_strength(cards: &str, is_joker_enabled: bool) -> (usize, Vec<usize>) {
    let translated_hands = translate_card_values(cards, is_joker_enabled);

    let mut seen_vals: HashMap<usize, usize> = HashMap::new();
    for i in 0..translated_hands.len() {
        *seen_vals.entry(translated_hands[i]).or_insert(0) += 1;
    }
    let mut joker_count = 0;
    if is_joker_enabled {
        joker_count = *(seen_vals.get(&0).unwrap_or(&0));
    }
    match seen_vals.len() {
        4 => {
            if joker_count == 1 || joker_count == 2 {
                return (3, translated_hands);
            }
            return (1, translated_hands);
        }
        3 => {
            if seen_vals.values().any(|&v| v == 3) {
                if joker_count == 3 || joker_count == 1 {
                    return (5, translated_hands);
                }
                return (3, translated_hands);
            }
            if joker_count == 1 || joker_count == 2 {
                return (3 + joker_count, translated_hands);
            }

            return (2 + joker_count, translated_hands);
        }
        2 => {
            if seen_vals.values().any(|&v| v == 4) {
                if joker_count != 4 {
                    return (5 + joker_count, translated_hands);
                }
                return (6, translated_hands);
            }
            if joker_count > 0 {
                return (6, translated_hands);
            }
            return (4, translated_hands);
        }
        1 => return (6, translated_hands),
        _ => return (joker_count, translated_hands),
    };
}
