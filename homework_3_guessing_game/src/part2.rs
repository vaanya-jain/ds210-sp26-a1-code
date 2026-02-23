use std::mem::min_align_of;

use crate::player::{Player, PlayerTrait};
use crate::strategies::Strategy;

pub struct Part2 {}

// Terrible strategy: ask if the number is min, otherwise return max.
impl Strategy for Part2 {
    fn guess_the_number(player: &mut Player, min: u32, max: u32) -> u32 {
    

        let mut min1 = min;
        let mut max1 = max;
        while min1<max1{
            let mut mid = (min1 + max1)/2;
            let val = player.ask_to_compare(mid);
            match val {
                0 => return mid,
                1 => min1 = mid+ 1,
                -1=> max1 = mid -1,
                 _ => println!("no")


            }
            
        }
        min1

     }
}
