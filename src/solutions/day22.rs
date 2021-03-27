use crate::AoCDay;
use std::collections::VecDeque;

pub struct Code;

/**
 * Helper function to parse a given string (the file contents) into the appropriate vectors
 */
fn parse_file_to_vecs(file_content: String) -> (VecDeque<usize>, VecDeque<usize>) {
    //Need to use nightly to run split_once as it's unstable in stable (???)
    let (player1, player2) = file_content.split_once("\n\n").unwrap();
    //Get all p1 cards
    let p1cards: VecDeque<usize> = player1
        //Automatically split by line
        .lines()
        //The first one seems to say "Player1" so skip that
        .skip(1)
        //Parse each line into a number
        .map(|lc| lc.parse().unwrap())
        //Get all the numbers as a vec
        .collect();
    //Repeat for p2cards
    let p2cards: VecDeque<usize> = player2
        .lines()
        .skip(1)
        .map(|lc| lc.parse().unwrap())
        .collect();
    return (p1cards, p2cards);
}

/**
 * Helper function to play one round of the game.
 */
fn play_nonrec_round(p1cards: &mut VecDeque<usize>, p2cards: &mut VecDeque<usize>) {
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
fn play_all_nonrec_rounds(p1cards: &mut VecDeque<usize>, p2cards: &mut VecDeque<usize>){
    while !p1cards.is_empty() && !p2cards.is_empty(){
        play_nonrec_round(p1cards, p2cards);
    }
}

/**
 * Helper function to score a deck!
 */
fn score_deck(deck: VecDeque<usize>) -> usize{
    //OK, I have a deck... reverse, zip with positions (enumerate? Is that what rust calls it?), and then multiply, then sum?
    return deck
        .into_iter()
        .rev()
        .enumerate()
        .map(|(index, value)| (index + 1) * value)
        .sum()
}

impl AoCDay for Code {
    fn part1(&self, input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {
        let mut file_content: String = String::new();
        //To read in as string yields an
        let _size = input.read_to_string(&mut file_content);
        let (mut p1, mut p2) = parse_file_to_vecs(file_content);
        play_all_nonrec_rounds(&mut p1,&mut  p2);
        //OK, we don't know or care who the winner is?
        if p1.is_empty(){
            return score_deck(p2).to_string();
        }else{
            return score_deck(p1).to_string();
        }
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_simple_rounds() {
        let mut p1a: VecDeque<usize> = VecDeque::with_capacity(2);
        let mut p2a: VecDeque<usize> = VecDeque::with_capacity(2);
        p1a.push_front(1);
        p2a.push_front(2);
        play_nonrec_round(&mut p1a, &mut p2a);
        assert_eq!(p1a.pop_front(), None);
        assert_eq!(p2a.pop_front(), Some(2));
        assert_eq!(p2a.pop_front(), Some(1));
        assert_eq!(p2a.pop_front(), None);
        //OK, now try the other way around!
        
        let mut p1b: VecDeque<usize> = VecDeque::with_capacity(2);
        let mut p2b: VecDeque<usize> = VecDeque::with_capacity(2);
        p1b.push_front(9);
        p2b.push_front(5);
        play_nonrec_round(&mut p1b, &mut p2b);
        assert_eq!(p2b.pop_front(), None);
        assert_eq!(p1b.pop_front(), Some(9));
        assert_eq!(p1b.pop_front(), Some(5));
        assert_eq!(p1b.pop_front(), None);
    }

    #[test]
    fn test_example_first_round(){
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
    fn test_example_two_rounds(){
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
    fn test_example_all_rounds(){
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        play_all_nonrec_rounds(&mut p1, &mut p2);
        //Thankfully we have the answer from the page!
        //p1 should be empty!
        assert_eq!(p1.pop_front(), None);
        //And p2 should be 3, 2, 10, 6, 8, 5, 9, 4, 7, 1
        assert_eq!(p2.pop_front(),Some(3));
        assert_eq!(p2.pop_front(),Some(2));
        assert_eq!(p2.pop_front(),Some(10));
        assert_eq!(p2.pop_front(),Some(6));
        assert_eq!(p2.pop_front(),Some(8));
        assert_eq!(p2.pop_front(),Some(5));
        assert_eq!(p2.pop_front(),Some(9));
        assert_eq!(p2.pop_front(),Some(4));
        assert_eq!(p2.pop_front(),Some(7));
        assert_eq!(p2.pop_front(),Some(1));
        assert_eq!(p2.pop_front(),None)
    }

    #[test]
    fn test_scoring(){
        let file_conts = include_str!("../../inputs/d22-test").to_string();
        let (mut p1, mut p2) = parse_file_to_vecs(file_conts);
        play_all_nonrec_rounds(&mut p1, &mut p2);
        //According to the page, the results should be... 0 for p1, and 306 for p2?
        assert_eq!(score_deck(p1),0);
        assert_eq!(score_deck(p2),306);
    }
}
