use crate::AoCDay;
use std::collections::hash_map::DefaultHasher;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};

pub struct Code;

impl AoCDay for Code {
    fn part1(&self, input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let mut file_content: String = String::new();
        //To read in as string yields a a size... Which we ignore (bad rust!)
        let _size = input.read_to_string(&mut file_content);
        //Use helper functions (see below) - these make it easier to unit test (see the bottom of the page!)
        let (mut p1, mut p2) = parse_file_to_vecs(file_content);
        play_all_nonrec_rounds(&mut p1, &mut p2);
        //OK, we don't know or care who the winner is? Just need the final score
        if p1.is_empty() {
            return score_deck(p2).to_string();
        } else {
            return score_deck(p1).to_string();
        }
    }

    fn part2(&self, input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        let mut file_content: String = String::new();
        //To read in as string yields a a size... Which we ignore (bad rust!)
        let _size = input.read_to_string(&mut file_content);
        //Use helper functions (see below) - these make it easier to unit test (see the bottom of the page!)
        let (mut p1, mut p2) = parse_file_to_vecs(file_content);
        let mut seen: BTreeSet<u64> = BTreeSet::new();
        play_all_rec_rounds(&mut p1, &mut p2, &mut seen);
        if p1.is_empty() {
            println!("2: {:?}", p2);
            return score_deck(p2).to_string();
        } else {
            println!("1: {:?}", p1);
            return score_deck(p1).to_string();
        }
    }
}

/**
 * Helper function to parse a given string (the file contents) into the appropriate vectors
 */
fn parse_file_to_vecs(file_content: String) -> (VecDeque<u8>, VecDeque<u8>) {
    //Need to use nightly to run split_once as it's unstable in stable (???)
    let (player1, player2) = file_content.split_once("\n\n").unwrap();
    //Get all p1 cards
    let p1cards: VecDeque<u8> = player1
        //Automatically split by line
        .lines()
        //The first one seems to say "Player1" so skip that
        .skip(1)
        //Parse each line into a number
        .map(|lc| lc.parse().unwrap())
        //Get all the numbers as a vec
        .collect();
    //Repeat for p2cards
    let p2cards: VecDeque<u8> = player2
        .lines()
        .skip(1)
        .map(|lc| lc.parse().unwrap())
        .collect();
    return (p1cards, p2cards);
}

/**
 * Helper function to play one round of the game.
 */
fn play_nonrec_round(p1cards: &mut VecDeque<u8>, p2cards: &mut VecDeque<u8>) {
    let card1 = p1cards.pop_front().unwrap();
    let card2 = p2cards.pop_front().unwrap();
    if card1 == card2 {
        panic!("Not mentioned in docs, so assume should not occur!");
    } else if card1 > card2 {
        p1cards.push_back(card1);
        p1cards.push_back(card2);
    } else {
        p2cards.push_back(card2);
        p2cards.push_back(card1);
    }
}

/**
 * Helper function to play all rounds of the game
 */
fn play_all_nonrec_rounds(p1cards: &mut VecDeque<u8>, p2cards: &mut VecDeque<u8>) {
    while !p1cards.is_empty() && !p2cards.is_empty() {
        play_nonrec_round(p1cards, p2cards);
    }
}

/**
 * Helper function to score a deck!
 */
fn score_deck(deck: VecDeque<u8>) -> usize {
    //OK, I have a deck... reverse, zip with positions (enumerate? Is that what rust calls it?), and then multiply, then sum?
    return deck
        .into_iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, value)| acc + (index + 1) * (value as usize));
}

/// All possible recursive game results
enum RecursiveTypes {
    ///We've done this game before, don't do it again
    InfinityBreak,
    ///We are able to make a sub game
    SubGame,
    ///EITHER of P1 or P2 didn't have enough cards. Use the bool to denote if the winner is P1 (true) or P2 (false)
    NotEnough(bool),
}
/**
 * Helper function to determine what type of recursive round we are looking at
 */
fn determine_rec_round_type(
    p1cards: &mut VecDeque<u8>,
    p2cards: &mut VecDeque<u8>,
    seen_hands: &mut BTreeSet<u64>,
) -> RecursiveTypes {
    let mut hasher = DefaultHasher::new();
    //TODO: see if there is a way to avoid the clone here... Memory!
    //Create the tuple for checking seen hand configs
    if false{
        let tpl = (p1cards.clone(), p2cards.clone());
        tpl.hash(&mut hasher);
    }else{
        //See https://dev.to/neilgall/comment/19fh4
        p1cards.hash(&mut hasher);
        //p2cards.hash(&mut hasher);
    }
    let hash = hasher.finish();
    //Have we seen it?
    if p1cards.is_empty() || p2cards.is_empty() {
        panic!("Don't pass empty vecs to me!!!")
    } else if seen_hands.contains(&hash) {
        //Yes, infinity break time!
        return RecursiveTypes::InfinityBreak;
    } else {
        //No, then add it and work out what we are doing!
        seen_hands.insert(hash);
        let &card1 = p1cards.front().unwrap();
        let &card2 = p2cards.front().unwrap();
        if ((p1cards.len()) > card1.into()) && ((p2cards.len()) > card2.into()) {
            return RecursiveTypes::SubGame;
        } else if card1 > card2 {
            return RecursiveTypes::NotEnough(true);
        } else if card1 == card2 {
            panic!("We have no instructions for the same card!")
        } else {
            return RecursiveTypes::NotEnough(false);
        }
    }
}

/**
 * Helper functions - not for testing but so I don't get caught making the cards go the wrong way (also DRY)
 * Used in recursive version
 * Compiler will probably inline
 */
fn p1wins_rec(p1cards: &mut VecDeque<u8>, p2cards: &mut VecDeque<u8>) {
    let card1 = p1cards.pop_front().unwrap();
    let card2 = p2cards.pop_front().unwrap();
    //
    p1cards.push_back(card1);
    p1cards.push_back(card2);
}
fn p2wins_rec(p1cards: &mut VecDeque<u8>, p2cards: &mut VecDeque<u8>) {
    let card1 = p1cards.pop_front().unwrap();
    let card2 = p2cards.pop_front().unwrap();
    //Push these the correct way around since 2 won!
    p2cards.push_back(card2);
    p2cards.push_back(card1);
}

/// Return true if winner is P1, return false if winner is P2!
fn play_all_rec_rounds(
    p1cards: &mut VecDeque<u8>,
    p2cards: &mut VecDeque<u8>,
    seen: &mut BTreeSet<u64>,
) -> bool {
    //OK, let's loop!
    while (!p1cards.is_empty()) && (!p2cards.is_empty()) {
        let result: RecursiveTypes = determine_rec_round_type(p1cards, p2cards, seen);
        match result {
            RecursiveTypes::InfinityBreak => p1wins_rec(p1cards, p2cards),
            RecursiveTypes::NotEnough(true) => p1wins_rec(p1cards, p2cards),
            RecursiveTypes::NotEnough(false) => p2wins_rec(p1cards, p2cards),
            RecursiveTypes::SubGame => {
                //OK, play a sub game and go from there!
                let &card1 = p1cards.front().unwrap();
                let &card2 = p2cards.front().unwrap();
                //Now we play with the next card1 cards of p1cards, and card2 cards of part2
                //And then the winner is determined from there!
                let mut new_p1 = p1cards.range(1..card1.into()).copied().collect::<VecDeque<_>>();
                let mut new_p2 = p2cards.range(1..card2.into()).copied().collect::<VecDeque<_>>();
                if play_all_rec_rounds(&mut new_p1, &mut new_p2, seen) {
                    p1wins_rec(p1cards, p2cards)
                } else {
                    p2wins_rec(p1cards, p2cards)
                }
            }
        }
    }
    //If p2 cards is empty, then p1 wins (true), otherwise p2 wins (false)
    return p2cards.is_empty();
}

#[cfg(test)]
mod tests_part_2 {
    use super::*;

    #[test]
    fn test_rec_type_calc() {
        //OK, test all 4 known valid scenarios
        let mut p1a: VecDeque<u8> = VecDeque::with_capacity(1);
        let mut p2a: VecDeque<u8> = VecDeque::with_capacity(1);
        let mut seen: BTreeSet<u64> = BTreeSet::new();
        p1a.push_front(1);
        p2a.push_front(2);
        //First time should yeild NotEnoughSoP2
        assert!(matches!(
            determine_rec_round_type(&mut p1a, &mut p2a, &mut seen),
            RecursiveTypes::NotEnough(false)
        ));
        //If we try the same thing again, we should get a result of InfinityBreak
        assert_eq!(seen.len(), 1);
        assert!(matches!(
            determine_rec_round_type(&mut p1a, &mut p2a, &mut seen),
            RecursiveTypes::InfinityBreak
        ));
        //OK, try and do the reverse this time and should get P2 (just swap args so labelling is iffy but this is a hackjob!)
        let res = determine_rec_round_type(&mut p2a, &mut p1a, &mut seen);
        assert!(matches!(res, RecursiveTypes::NotEnough(true)));
        //Again, repeat = InfinityBreak
        assert_eq!(seen.len(), 2);
        assert!(matches!(
            determine_rec_round_type(&mut p2a, &mut p1a, &mut seen),
            RecursiveTypes::InfinityBreak
        ));
    }

    #[test]
    fn test_play_all_rec_rounds() {
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        let mut seen: BTreeSet<u64> = BTreeSet::new();
        assert_eq!(play_all_rec_rounds(&mut p1, &mut p2, &mut seen), false);
        // I should be able to test the vecs themselves too!
        //1 Should be empty
        assert_eq!(p1.pop_front(), None);
        //2 should be 7, 5, 6, 2, 4, 1, 10, 8, 9, 3
        assert_eq!(p2.pop_front(), Some(7));
        assert_eq!(p2.pop_front(), Some(5));
        assert_eq!(p2.pop_front(), Some(6));
        assert_eq!(p2.pop_front(), Some(2));
        assert_eq!(p2.pop_front(), Some(4));
        assert_eq!(p2.pop_front(), Some(1));
        assert_eq!(p2.pop_front(), Some(10));
        assert_eq!(p2.pop_front(), Some(8));
        assert_eq!(p2.pop_front(), Some(9));
        assert_eq!(p2.pop_front(), Some(3));
        assert_eq!(p2.pop_front(), None)
    }

    #[test]
    fn test_all_rec_score() {
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        let mut seen: BTreeSet<u64> = BTreeSet::new();
        assert_eq!(play_all_rec_rounds(&mut p1, &mut p2, &mut seen), false);
        assert_eq!(score_deck(p2), 291);
        assert_eq!(score_deck(p1), 0);
    }
}

#[cfg(test)]
mod tests_part_1 {
    use super::{
        parse_file_to_vecs, play_all_nonrec_rounds, play_nonrec_round, score_deck, VecDeque,
    };

    #[test]
    fn test_simple_rounds() {
        let mut p1a: VecDeque<u8> = VecDeque::with_capacity(2);
        let mut p2a: VecDeque<u8> = VecDeque::with_capacity(2);
        p1a.push_front(1);
        p2a.push_front(2);
        play_nonrec_round(&mut p1a, &mut p2a);
        assert_eq!(p1a.pop_front(), None);
        assert_eq!(p2a.pop_front(), Some(2));
        assert_eq!(p2a.pop_front(), Some(1));
        assert_eq!(p2a.pop_front(), None);
        //OK, now try the other way around!
        let mut p1b: VecDeque<u8> = VecDeque::with_capacity(2);
        let mut p2b: VecDeque<u8> = VecDeque::with_capacity(2);
        p1b.push_front(9);
        p2b.push_front(5);
        play_nonrec_round(&mut p1b, &mut p2b);
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
        play_nonrec_round(&mut p1, &mut p2);

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
        play_nonrec_round(&mut p1, &mut p2);
        play_nonrec_round(&mut p1, &mut p2);
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
    fn test_example_all_rounds() {
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        play_all_nonrec_rounds(&mut p1, &mut p2);
        //Thankfully we have the answer from the page!
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

    #[test]
    fn test_scoring() {
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        play_all_nonrec_rounds(&mut p1, &mut p2);
        //According to the page, the results should be... 0 for p1, and 306 for p2?
        assert_eq!(score_deck(p1), 0);
        assert_eq!(score_deck(p2), 306);
    }
}

#[cfg(test)]
mod tests_parts_both {
    use super::parse_file_to_vecs;

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
