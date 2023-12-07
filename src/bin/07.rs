use std::cmp::{Ordering, Reverse};
use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Hand {
    cards: [u8; 5],
    score: u8,
    bid: u32,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.score > other.score {
            return Ordering::Greater;
        }
        if self.score < other.score {
            return Ordering::Less;
        }
        for (ca, cb) in self.cards.iter().zip(other.cards.iter()) {
            if ca > cb {
                return Ordering::Greater;
            }
            if cb > ca {
                return Ordering::Less;
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn char_to_nb(c: char, jokers: bool) -> u8 {
    match c.to_digit(10) {
        Some(d) => {
            d as u8 - {
                if jokers {
                    1
                } else {
                    2
                }
            }
        }
        None => match c {
            'A' => 12,
            'K' => 11,
            'Q' => 10,
            'J' => {
                if jokers {
                    0
                } else {
                    9
                }
            }
            'T' => {
                if jokers {
                    9
                } else {
                    8
                }
            }
            _ => unreachable!(),
        },
    }
}

fn score_hand(h: &Vec<char>, jokers: bool) -> (u8, Vec<u8>) {
    let mut hand: Vec<u8> = vec![0; 13];
    let mut nb_jokers: u8 = 0;
    for c in h.iter() {
        if *c == 'J' && jokers {
            nb_jokers += 1;
            continue;
        }
        hand[char_to_nb(*c, jokers) as usize] += 1;
    }
    hand.sort_by_key(|x| Reverse(*x));

    let score = match (hand[0], nb_jokers) {
        (5, _) => 50,
        (_, 5) => 50,
        (4, _) => 10 * (4 + nb_jokers),
        (3, _) => {
            if hand[1] == 2 {
                32
            } else {
                10 * (3 + nb_jokers)
            }
        }
        (2, _) => {
            if hand[1] == 2 {
                22 + 10 * nb_jokers
            } else {
                10 * (2 + nb_jokers)
            }
        }
        (1, _) => 10 * (1 + nb_jokers),
        (_, _) => 0,
    };
    return (score, h.iter().map(|x| char_to_nb(*x, jokers)).collect());
}

fn main() {
    let mut hands_p1: Vec<Hand> = Vec::new();
    let mut hands_p2: Vec<Hand> = Vec::new();
    for line in read_to_string("input/07").unwrap().lines() {
        let mut it = line.split(" ");
        let chars = it.next().unwrap().chars().collect();
        let bid = it.next().unwrap().parse::<u32>().unwrap();
        let s1 = score_hand(&chars, false);
        hands_p1.push(Hand {
            cards: [s1.1[0], s1.1[1], s1.1[2], s1.1[3], s1.1[4]],
            score: s1.0,
            bid,
        });
        let s2 = score_hand(&chars, true);
        hands_p2.push(Hand {
            cards: [s2.1[0], s2.1[1], s2.1[2], s2.1[3], s2.1[4]],
            score: s2.0,
            bid,
        });
    }

    hands_p1.sort();
    hands_p2.sort();

    println!(
        "Part 1: {}\nPart 2: {}",
        hands_p1.iter().enumerate().fold(0, |acc, (i, card)| acc
            + ((i as u64 + 1) * (card.bid as u64))),
        hands_p2.iter().enumerate().fold(0, |acc, (i, card)| acc
            + ((i as u64 + 1) * (card.bid as u64)))
    )
}
