use std::mem::min_align_of;

use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
        // YOUR SOLUTION GOES HERE.
        let mut low = min;
        let mut high = max;
        while low < high {
            let mid = low + (high - low) / 2;

            match player.ask_to_compare(mid) {
                0 => return mid,
                -1 => high = mid,
                1 => low = mid + 1,
                _ => unreachable!("ask_to_compare should only return -1, 0, or 1"),
            }
        }

        low
    }
}
