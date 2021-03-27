use crate::AoCDay;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::collections::VecDeque;

pub struct Code;

type Deck = VecDeque<u8>;

impl AoCDay for Code {
    fn part1(&self, input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let mut file_content: String = String::new();
        //To read in as string yields a a size... Which we ignore (bad rust!)
        let _size = input.read_to_string(&mut file_content);
        //Use helper functions (see below) - these make it easier to unit test (see the bottom of the page!)
        let (mut p1, mut p2) = parse_file_to_vecs(file_content);
        let winner = play_all_nonrec_rounds(&mut p1, &mut p2);
        //OK, we don't know or care who the winner is? Just need the final score
        return match winner {
            Winner::P1 => score_deck(p1).to_string(),
            Winner::P2 => score_deck(p2).to_string(),
        };
    }

    fn part2(&self, input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let mut file_content: String = String::new();
        //To read in as string yields a a size... Which we ignore (bad rust!)
        let _size = input.read_to_string(&mut file_content);
        //Use helper functions (see below) - these make it easier to unit test (see the bottom of the page!)
        let (mut p1, mut p2) = parse_file_to_vecs(file_content);
        let winner = play_all_rec_rounds(&mut p1, &mut p2);
        //Again, don't care who, just that
        return match winner {
            Winner::P1 => score_deck(p1).to_string(),
            Winner::P2 => score_deck(p2).to_string(),
        };
    }
}

enum Winner {
    P1,
    P2,
}

/**
 * Helper function to parse a given string (the file contents) into the appropriate vectors
 */
fn parse_file_to_vecs(file_content: String) -> (Deck, Deck) {
    //Need to use nightly to run split_once as it's unstable in stable (???)
    let (player1, player2) = file_content.split_once("\n\n").unwrap();
    //Get all p1 cards
    let p1cards: Deck = player1
        //Automatically split by line
        .lines()
        //The first one seems to say "Player1" so skip that
        .skip(1)
        //Parse each line into a number
        .map(|lc| lc.parse().unwrap())
        //Get all the numbers as a vec
        .collect();
    //Repeat for p2cards
    let p2cards: Deck = player2
        .lines()
        .skip(1)
        .map(|lc| lc.parse().unwrap())
        .collect();
    return (p1cards, p2cards);
}

/**
 * Helper function for comparing cards
 */
fn compare_cards(p1: u8, p2: u8) -> Winner {
    match p1.cmp(&p2) {
        Ordering::Greater => Winner::P1,
        Ordering::Less => Winner::P2,
        Ordering::Equal => {
            panic!("Not mentioned in docs, so assume should not occur!");
        }
    }
}

/**
 * Helper function for working on the decks so I don't mess up
 */
fn apply_winner(p1deck: &mut Deck, p2deck: &mut Deck, p1card: u8, p2card: u8, winner: Winner) {
    match winner {
        Winner::P1 => {
            p1deck.push_back(p1card);
            p1deck.push_back(p2card);
        }
        Winner::P2 => {
            p2deck.push_back(p2card);
            p2deck.push_back(p1card);
        }
    }
}

/**
 * Helper function to play all rounds of the game
 */
fn play_all_nonrec_rounds(p1cards: &mut Deck, p2cards: &mut Deck) -> Winner {
    while !p1cards.is_empty() && !p2cards.is_empty() {
        let card1 = p1cards.pop_front().unwrap();
        let card2 = p2cards.pop_front().unwrap();
        apply_winner(p1cards, p2cards, card1, card2, compare_cards(card1, card2))
    }
    return if p2cards.is_empty() {
        Winner::P1
    } else {
        Winner::P2
    };
}

/**
 * Helper function to play rec rounds
 */
fn play_all_rec_rounds(p1cards: &mut Deck, p2cards: &mut Deck) -> Winner {
    let mut seen_decks_p1: HashSet<Deck> = HashSet::new();
    let mut seen_decks_p2: HashSet<Deck> = HashSet::new();
    while !p1cards.is_empty() && !p2cards.is_empty() {
        if !seen_decks_p1.insert(p1cards.clone()) && !seen_decks_p2.insert(p2cards.clone()) {
            return Winner::P1;
        }
        let card1 = p1cards.pop_front().unwrap();
        let card2 = p2cards.pop_front().unwrap();
        if p1cards.len() >= card1 as usize && p2cards.len() >= card2 as usize {
            let mut new_p1: Deck = p1cards.iter().take(card1 as usize).cloned().collect();
            let mut new_p2: Deck = p2cards.iter().take(card2 as usize).cloned().collect();
            let winner = play_all_rec_rounds(&mut new_p1, &mut new_p2);
            apply_winner(p1cards, p2cards, card1, card2, winner);
        } else {
            apply_winner(p1cards, p2cards, card1, card2, compare_cards(card1, card2))
        }
    }
    return if p2cards.is_empty() {
        Winner::P1
    } else {
        Winner::P2
    };
}

/**
 * Helper function to score a deck!
 */
fn score_deck(deck: Deck) -> usize {
    //OK, I have a deck... reverse, zip with positions (enumerate? Is that what rust calls it?), and then multiply, then sum?
    return deck
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, value)| {
            acc + (index + 1) * (value as usize)
        });
}

#[cfg(test)]
mod tests_part_2 {
    use super::*;
    #[test]
    fn test_example_rec_rounds() {
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        let winner = play_all_rec_rounds(&mut p1, &mut p2);
        assert!(matches!(winner, Winner::P2));
        assert_eq!(score_deck(p2), 291);
    }
}

#[cfg(test)]
mod tests_part_1 {
    use super::{parse_file_to_vecs, play_all_nonrec_rounds, Winner};

    #[test]
    fn test_example_all_rounds() {
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        let winner = play_all_nonrec_rounds(&mut p1, &mut p2);
        //Thankfully we have the answer from the page!
        //Winner is p2
        assert!(matches!(winner, Winner::P2));
        //p1 should be empty!
        assert_eq!(p1.pop_front(), None);
        //And p2 should be 3, 2, 10, 6, 8, 5, 9, 4, 7, 1
        assert_eq!(p2.pop_front(), Some(3));
        assert_eq!(p2.pop_front(), Some(2));
        assert_eq!(p2.pop_front(), Some(10));
        assert_eq!(p2.pop_front(), Some(6));
        assert_eq!(p2.pop_front(), Some(8));
        assert_eq!(p2.pop_front(), Some(5));
        assert_eq!(p2.pop_front(), Some(9));
        assert_eq!(p2.pop_front(), Some(4));
        assert_eq!(p2.pop_front(), Some(7));
        assert_eq!(p2.pop_front(), Some(1));
        assert_eq!(p2.pop_front(), None)
    }
}

#[cfg(test)]
mod tests_parts_both {
    use super::{
        apply_winner, compare_cards, parse_file_to_vecs, score_deck, Deck, VecDeque, Winner,
    };

    /**
     * Helper function to play one round of the core game.
     */
    fn play_round(p1cards: &mut Deck, p2cards: &mut Deck) {
        let card1 = p1cards.pop_front().unwrap();
        let card2 = p2cards.pop_front().unwrap();
        apply_winner(p1cards, p2cards, card1, card2, compare_cards(card1, card2))
    }

    #[test]
    fn test_scoring() {
        //[3, 2, 10, 6, 8, 5, 9, 4, 7, 1]
        let score_me: Deck = VecDeque::from(vec![3, 2, 10, 6, 8, 5, 9, 4, 7, 1]);
        assert_eq!(score_deck(score_me), 306)
    }

    #[test]
    fn test_simple_rounds() {
        let mut p1a: Deck = VecDeque::with_capacity(2);
        let mut p2a: Deck = VecDeque::with_capacity(2);
        p1a.push_front(1);
        p2a.push_front(2);
        play_round(&mut p1a, &mut p2a);
        assert_eq!(p1a.pop_front(), None);
        assert_eq!(p2a.pop_front(), Some(2));
        assert_eq!(p2a.pop_front(), Some(1));
        assert_eq!(p2a.pop_front(), None);
        //OK, now try the other way around!
        let mut p1b: Deck = VecDeque::with_capacity(2);
        let mut p2b: Deck = VecDeque::with_capacity(2);
        p1b.push_front(9);
        p2b.push_front(5);
        play_round(&mut p1b, &mut p2b);
        assert_eq!(p2b.pop_front(), None);
        assert_eq!(p1b.pop_front(), Some(9));
        assert_eq!(p1b.pop_front(), Some(5));
        assert_eq!(p1b.pop_front(), None);
    }

    #[test]
    fn test_example_first_round() {
        /*
        Should be 9 vs 5, so playing one round should push 9 then 5 to the end of p1
        */
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        play_round(&mut p1, &mut p2);

        //1
        assert_eq!(p1.pop_front(), Some(2));
        assert_eq!(p1.pop_front(), Some(6));
        assert_eq!(p1.pop_front(), Some(3));
        assert_eq!(p1.pop_front(), Some(1));
        assert_eq!(p1.pop_front(), Some(9));
        assert_eq!(p1.pop_front(), Some(5));
        assert_eq!(p1.pop_front(), None);

        //2
        assert_eq!(p2.pop_front(), Some(8));
        assert_eq!(p2.pop_front(), Some(4));
        assert_eq!(p2.pop_front(), Some(7));
        assert_eq!(p2.pop_front(), Some(10));
        assert_eq!(p2.pop_front(), None);
    }
    #[test]
    fn test_example_two_rounds() {
        /*
        Should be 9 vs 5, so playing one round should push 9 then 5 to the end of p1
        Then it's 2 vs 8, so it should push 8 then 2 to p2
        */
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        play_round(&mut p1, &mut p2);
        play_round(&mut p1, &mut p2);
        //1
        assert_eq!(p1.pop_front(), Some(6));
        assert_eq!(p1.pop_front(), Some(3));
        assert_eq!(p1.pop_front(), Some(1));
        assert_eq!(p1.pop_front(), Some(9));
        assert_eq!(p1.pop_front(), Some(5));
        assert_eq!(p1.pop_front(), None);

        //2
        assert_eq!(p2.pop_front(), Some(4));
        assert_eq!(p2.pop_front(), Some(7));
        assert_eq!(p2.pop_front(), Some(10));
        assert_eq!(p2.pop_front(), Some(8));
        assert_eq!(p2.pop_front(), Some(2));
        assert_eq!(p2.pop_front(), None);
    }

    #[test]
    fn test_apply_winner() {
        let mut p1a: Deck = VecDeque::with_capacity(1);
        let mut p2a: Deck = VecDeque::with_capacity(3);
        p1a.push_front(1);
        p2a.push_front(2);

        apply_winner(&mut p1a, &mut p2a, 3, 4, Winner::P2);
        assert_eq!(p2a.len(), 3);
        assert_eq!(p2a.pop_back(), Some(3));
        assert_eq!(p2a.pop_back(), Some(4));
        assert_eq!(p2a.pop_back(), Some(2));

        let mut p1b: Deck = VecDeque::with_capacity(3);
        let mut p2b: Deck = VecDeque::with_capacity(2);
        p1b.push_front(1);
        p2b.push_front(2);
        apply_winner(&mut p1b, &mut p2b, 4, 3, Winner::P1);
        assert_eq!(p1b.len(), 3);
        assert_eq!(p1b.pop_back(), Some(3));
        assert_eq!(p1b.pop_back(), Some(4));
        assert_eq!(p1b.pop_back(), Some(1));

        let mut p1c: Deck = VecDeque::with_capacity(3);
        let mut p2c: Deck = VecDeque::with_capacity(2);
        p1c.push_front(1);
        p2c.push_front(2);
        apply_winner(&mut p1c, &mut p2c, 3, 4, Winner::P1);
        assert_eq!(p1c.len(), 3);
        assert_eq!(p1c.pop_back(), Some(4));
        assert_eq!(p1c.pop_back(), Some(3));
        assert_eq!(p1c.pop_back(), Some(1));
    }

    #[test]
    fn test_card_compare() {
        assert!(matches!(compare_cards(1, 2), Winner::P2));
        assert!(matches!(compare_cards(5, 2), Winner::P1));
    }

    #[test]
    fn parse_file_to_vs_test() {
        // Import the test deck given
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        //Verify the contents for both p1 and p2!
        assert_eq!(p1.pop_front(), Some(9));
        assert_eq!(p1.pop_front(), Some(2));
        assert_eq!(p1.pop_front(), Some(6));
        assert_eq!(p1.pop_front(), Some(3));
        assert_eq!(p1.pop_front(), Some(1));
        assert_eq!(p1.pop_front(), None);
        assert_eq!(p2.pop_front(), Some(5));
        assert_eq!(p2.pop_front(), Some(8));
        assert_eq!(p2.pop_front(), Some(4));
        assert_eq!(p2.pop_front(), Some(7));
        assert_eq!(p2.pop_front(), Some(10));
        assert_eq!(p2.pop_front(), None);
    }
}
