use std::usize;

use crate::AoCDay;
pub struct Code;

type Loops = usize;
type Encryption = usize;
const SUBJECT: usize = 7;
const MODULAR: usize = 20201227;
const CARD_PUBLIC_KEY: usize = 9717666;
const DOOR_PUBLIC_KEY: usize = 20089533;

impl AoCDay for Code {
    fn part1(&self, _input: &mut dyn std::io::Read, _extra_argss: &[String]) -> String {

        return solve(CARD_PUBLIC_KEY, DOOR_PUBLIC_KEY).0.to_string();
    }

    fn part2(&self, _input: &mut dyn std::io::Read, _extra_args: &[String]) -> String {
        return "".to_string();
    }
}


//I guess we solve from the perspective of 'A'?
// As 'A' I need to work out my own public key from 1
// And that same number of loops turns B's key into an encryption key?
const fn solve(key_a: Encryption, key_b: Encryption) -> (Encryption, Loops){
    
    let mut key_a_tracker: Encryption = 1;
    let mut encyrption_key: Encryption = 1;
    let mut num_loops: Loops = 0;
    // Keep looping until we can make 1 become the first key using SUBJECT and MODULAR;
    while key_a_tracker != key_a {
        //Change the tracker to get closer to my key
        key_a_tracker = key_a_tracker * SUBJECT % MODULAR;
        //Change the enc key using B's key????
        encyrption_key = encyrption_key * key_b % MODULAR;
        //Count the number of loops (for tests)
        num_loops += 1;
    }
    return (encyrption_key,num_loops);
}

#[cfg(test)]
mod tests{
    use crate::day25::solve;

    use super::{Encryption, Loops};
    const DOOR_KEY: Encryption = 17807724;
    const CARD_KEY: Encryption = 5764801;

    const CARD_LOOPS: Loops = 8;
    const DOOR_LOOPS: Loops = 11;
    const RESULT: Encryption = 14897079;

    #[test]
    fn test_loops(){
        assert_eq!(solve(CARD_KEY, DOOR_KEY).1, CARD_LOOPS);
        assert_eq!(solve(DOOR_KEY, CARD_KEY).1, DOOR_LOOPS);
    }
    #[test]
    fn test_enc(){
        assert_eq!(solve(CARD_KEY, DOOR_KEY).0, RESULT);
        assert_eq!(solve(DOOR_KEY, CARD_KEY).0, RESULT);
    }
}