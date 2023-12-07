use std::{collections::HashMap, fs};

use crate::{AdventError, ExclusivePart};

const INPUT_FILE: &str = "./resources/day07_input.txt";

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    let total_winnings = get_total_winnings(false).map_err(|err| AdventError::Other(err))?;

    Ok(total_winnings.to_string())
}

fn part_two() -> Result<String, AdventError> {
    let total_winnings = get_total_winnings(true).map_err(|err| AdventError::Other(err))?;

    Ok(total_winnings.to_string())
}

fn get_total_winnings(joker_mode: bool) -> Result<u64, String> {
    // read input file
    let input = fs::read_to_string(INPUT_FILE).map_err(|err| err.to_string())?;

    let hand_and_bids: Result<Vec<HandAndBid>, String> = input
        .lines()
        .map(|line| {
            let (unparsed_hand, unparsed_bid) = line.split_once(" ").unwrap();

            let hand = Hand::from_str(unparsed_hand, joker_mode)?;
            let bid = unparsed_bid.parse::<u64>().map_err(|err| err.to_string())?;

            Ok(HandAndBid { hand, bid })
        })
        .collect();
    let mut hand_and_bids = hand_and_bids?;

    // sort and rank hands
    hand_and_bids.sort();
    let hand_bid_and_ranks: Vec<HandBidAndRank> = hand_and_bids
        .into_iter()
        .enumerate()
        .map(|(index, hand_and_bid)| HandBidAndRank {
            hand: hand_and_bid.hand,
            bid: hand_and_bid.bid,
            rank: index as u64 + 1,
        })
        .collect();

    // for hand in hand_bid_and_ranks.iter() {
    //     println!(
    //         "hand: {:?}\n  bid: {:?}, rank: {:?}, winnings: {:?}",
    //         hand.hand,
    //         hand.bid,
    //         hand.rank,
    //         hand.winnings()
    //     );
    // }

    let total_winnings = hand_bid_and_ranks
        .iter()
        .map(|hand| hand.winnings())
        .sum::<u64>();

    Ok(total_winnings)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandBidAndRank {
    hand: Hand,
    bid: u64,
    rank: u64,
}

impl HandBidAndRank {
    fn winnings(&self) -> u64 {
        self.bid * self.rank
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct HandAndBid {
    hand: Hand,
    bid: u64,
}

impl PartialOrd for HandAndBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.hand.cmp(&other.hand))
    }
}

impl Ord for HandAndBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    hand_type: HandType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard, // lowest
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind, // highest
}

impl Hand {
    fn from_str(s: &str, use_joker: bool) -> Result<Self, String> {
        let cards: Result<Vec<Card>, String> =
            s.chars().map(|c| Card::from_char(c, use_joker)).collect();
        let cards = cards?;
        let cards: [Card; 5] = cards.try_into().map_err(|_| "Bad hand!".to_string())?;

        let mut sets_map = HashMap::new();
        for card in cards {
            if sets_map.contains_key(&card) {
                *sets_map.get_mut(&card).unwrap() += 1;
            } else {
                sets_map.insert(card, 1);
            }
        }

        let joker_count = if use_joker {
            sets_map.remove(&Card::Joker).unwrap_or(0)
        } else {
            0
        };
        let mut sets = sets_map.into_values().collect::<Vec<u64>>();
        sets.sort();
        sets.reverse();

        if sets.len() > 0 {
            sets[0] += joker_count;
        } else {
            // this is a super-special case that only occurs in a set of five Jokers (JJJJJ)
            sets.push(joker_count);
        }

        let hand_type = Self::get_hand_type_from_sets(&sets);

        Ok(Hand { cards, hand_type })
    }

    fn get_hand_type_from_sets(sets: &Vec<u64>) -> HandType {
        match sets.len() {
            1 => HandType::FiveOfAKind,
            2 => match sets[0] {
                4 => HandType::FourOfAKind,
                3 => HandType::FullHouse,
                _ => panic!("Too many items in set ({:?})! How did this happen?", sets),
            },
            3 => match sets[0] {
                3 => HandType::ThreeOfAKind,
                2 => HandType::TwoPair,
                _ => panic!("Too many items in set ({:?})! How did this happen?", sets),
            },
            4 => HandType::OnePair,
            5 => HandType::HighCard,
            _ => panic!("Too many sets ({:?}) for hand! How did this happen?", sets),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                for i in 0..5 {
                    match self.cards[i].cmp(&other.cards[i]) {
                        std::cmp::Ordering::Equal => continue,
                        ord => return Some(ord),
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
            ord => return Some(ord),
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker,      // lowest in joker mode
    Number(u8), // lowest in non-joker mode
    Ten,
    Jack,
    Queen,
    King,
    Ace, // highest
}

impl Card {
    fn from_char(c: char, use_joker: bool) -> Result<Self, String> {
        match c {
            'J' => {
                if use_joker {
                    Ok(Card::Joker)
                } else {
                    Ok(Card::Jack)
                }
            }

            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Number(9)),
            '8' => Ok(Card::Number(8)),
            '7' => Ok(Card::Number(7)),
            '6' => Ok(Card::Number(6)),
            '5' => Ok(Card::Number(5)),
            '4' => Ok(Card::Number(4)),
            '3' => Ok(Card::Number(3)),
            '2' => Ok(Card::Number(2)),
            _ => Err(format!("Invalid card: {}", c)),
        }
    }
}
