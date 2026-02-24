use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part1 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part1 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        for guess in min..max {
            if player.ask_to_compare(guess) == 0 {
                return guess;
            }
        }
        min
    }
}
