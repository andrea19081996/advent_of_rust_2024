/* 
Day 3: Restoring the Nice List
The elves were in high spirits. For the first time in centuries, yesterday’s code review had eradicated every unnecessary heap allocation in Santa’s list-checking algorithm. “Finally,” yapped an elf sipping a Red Bull mocktail, “no more unnecessary allocations, no more clones”

The workshop buzzed with excitement as the DevOps elves live-streamed the successful merge on ElfHub. Even Blitzen was chill for once, reclining by the server rack, listening to lofi beats.

But the joy didn’t last.

Santa stormed in, his energy somewhere between a VC pitch gone wrong and a meme that didn’t land on X. His face was redder than Rudolph’s nose.

“WHY,” he roared, “IS THE NICE LIST COMPLETELY EMPTY?”

The elves froze.

“What do you mean, empty?” stammered an elf. “It compiled perfectly last night—”

Santa cut them off. “LOOK! Not a single kid on the Nice list. Did you break the weights? Are we back to random clones and allocations?!” He slammed his candy-cane laptop onto the nearest desk, the screen glaring with the issue.


Your Mission
Help the elves re-write the is_nice function in Rust. Santa needs the Nice list back before Christmas Eve.

The is_nice function accepts two arguments:

good_deeds: u32: The number of good deeds a kid has done.
bad_deeds: u32: The number of bad deeds a kid has done.
Calculating the ratio
To calculate the ratio, follow this logic:

ratio = good_deeds / (good_deeds + bad_deeds)
But there's a catch!
Bad deeds are weighted more heavily than good deeds (twice as much). So, the final ratio is calculated as:

ratio = good_deeds / (good_deeds + (2 * bad_deeds))
After you find the ratio, you'll need to check if the kid is nice. A kid is considered nice if the ratio is greater than or equal to 0.75, if nice return true, otherwise return false.

Santa’s counting on you. Save Christmas and keep the Nice list free of data breaches — and, hopefully, Santa himself.

Requirements
If both good_deeds and bad_deeds are 0, the kid is naughty by default.
The function should return a bool value.
*/

// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

// main was not requested in the exercise
pub fn main(){
    let good_deeds = 10;
    let bad_deeds = 2;
    let result = is_nice(good_deeds, bad_deeds);
    if result {
        println!("The children is nice, he/she has {} good deeds and {} bad deeds.", good_deeds, bad_deeds)
    } else {
        println!("The children is naughty! he/she has {} good deeds and {} bad deeds.", good_deeds, bad_deeds)
    }
    
}

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    // Calculate the ratio of good deeds to total deeds
    // Any ratio greater than or equal to 0.75 is considered nice
    // e.g. 10 good deeds and 2 bad deeds =
    // (10 * 1) / ((10 * 1) + (2 * 2)) = 10 / 14 = 0.714... (not nice)
    // If both good and bad deeds are 0, the child is naughty
    
    if good_deeds == 0 && bad_deeds == 0{
        false
    } else {
        (good_deeds as f32)*GOOD_WEIGHT / ((good_deeds as f32)*GOOD_WEIGHT + (bad_deeds as f32)*BAD_WEIGHT) >= 0.75
    }
}
