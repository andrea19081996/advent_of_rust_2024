/*
Day 4: Structifying the Naughty List
Santa burst into the dev lounge, chugging his third espresso. "Great job yesterday, nerds! The is_nice function? Chef's kiss. But now, I want structure. STRUCTURE! We’re going full-on Rustacean. I need a Kid struct—immediately!"

The elves nodded enthusiastically, their tiny laptops open, running Arch Linux with bspwm because, obviously, they were that kind of devs. One elf, started yapping, "But Santa, why a struct? Isn’t this just overengineered?"

Santa slammed the table, shaking an untouched tray of gluten-free cookies. "No! A struct means no more random strings floating around. We need to encapsulate a kid's data—name, and niceness score. Plus, we’ll need some methods to make sense of it all."

The dev elves scrambled to work. In no time, they sketched out the basic blueprint. Santa glanced at the screen. "Not bad. But I will need this extended later. Keep it modular, bros!"

The room fell silent as the elves realized the implications. This was just the beginning of Santa’s unhinged data modeling spree.

Your Task
The elves need your help to finish the Kid struct.

Here is what you need to do:

Add two variants to the Niceness enum: Nice and Naughty. Nice takes the number of good deeds.
Add two fields to the Kid struct: name of type String and niceness of type Niceness.
Move the is_nice function we created on Day 3 to an associated function of the Kid struct.
Finally, implement the new() associated function for the Kid struct.
*/
use std::fmt;

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
    Nice(u32),
    Naughty
}

pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
    pub name: String,
    pub niceness: Niceness
}

// need that in the main part, to show the niceness attribute
impl fmt::Display for Niceness {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Niceness::Nice(good_deeds) => write!(f, "Nice ({} good deeds)", good_deeds),
            Niceness::Naughty => write!(f, "Naughty"),
        }
    }
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Kid::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };
        
        Kid{name: name, niceness: niceness}
        // Kid{name, niceness}
    }

    // Move yesterday's function to an associated function in the struct
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool{
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }
    
        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
    
        let ratio = good_deeds / (good_deeds + bad_deeds);
    
        ratio >= 0.75
    }
}

pub fn main(){
    let good_deeds = 10;
    let bad_deeds = 2;
    let result = Kid::is_nice(good_deeds, bad_deeds);
    if result {
        println!("The children is nice, he/she has {} good deeds and {} bad deeds.", good_deeds, bad_deeds)
    } else {
        println!("The children is naughty! he/she has {} good deeds and {} bad deeds.", good_deeds, bad_deeds)
    }

    let good_deeds = 20;
    let bad_deeds = 1;
    let children_1 = Kid::new(String::from("John"), good_deeds, bad_deeds);
    match children_1.niceness {
        Niceness::Nice(_) => println!("The children is nice, he/she has {} good deeds and {} bad deeds.", children_1.niceness, bad_deeds),
        Niceness::Naughty => println!("The children is naughty! he/she has {} good deeds and {} bad deeds.", good_deeds, bad_deeds)
    }
}