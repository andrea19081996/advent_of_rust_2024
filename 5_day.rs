/*
Day 5: Parsing naughty-list.csv
The elves stared at their screens. They had just written the Kid struct and were testing it with Santa’s data.

But something was wrong with the data, Prancer leaned back, smirking. "We forgot something obvious, didn’t we? The data’s raw strings—we need to parse it first."

Alice,10,2
Bob,5,5
Charlie,1,9
"We need to create another function," Prancer continued. "to parse the CSV rows into Kid structs."

Blitzen slammed his mug down. "And since Santa put me in charge of this project, I’m naming the function. It’s going to be called parse_row."

An elf from the back muttered just loud enough to hear, "Ugh, he thinks he’s better than us because Santa made him lead."

Blitzen shot them a look. "I heard that. If you’ve got a better name, I’m listening."

Silence.

"Exactly. parse_row it is."

The Frustration
Blitzen paced. "We need a function that takes a CSV row, splits it, and converts it into a Kid. Name is easy—it stays a String. The good and bad deeds, though, need to be parsed to u32."

"But what if the row has garbage data?" asked an elf, holding up a note with Charlie,,9 scribbled on it.

Prancer rolled his eyes. "Obviously, we handle errors. No .unwrap() shortcuts."

The Task
Blitzen wants you to create an associated function for the Kid struct and name it parse_row. It should take a CSV row as a &str and return a Result<Kid, &'static str>. The function should:

Split the CSV row into parts.
Extract the name as a String.
Parse the second and third fields as u32 for good and bad deeds.
Finally create a Kid struct using the new() associated function we created earlier. 
*/
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
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
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1

        // 🎁 Your code here! 🎁
        // Split the row into parts separated by commas
        let parts: Vec<&str> = csv_row.split(',').collect();

        // Ensure there are exactly three parts (name, good_deeds, bad_deeds)
        if parts.len() != 3 {
            return Err("Invalid CSV format. Expected: Name,GoodDeeds,BadDeeds");
        }

        // Extract the name, good deeds, and bad deeds
        let name = parts[0].trim().to_string(); // Remove any extra whitespace

        // Parse good deeds and bad deeds into integers
        let good_deeds = parts[1]
            .trim()
            .parse::<u32>()
            .map_err(|_| "Failed to parse good deeds as a positive integer")?;

        let bad_deeds = parts[2]
            .trim()
            .parse::<u32>()
            .map_err(|_| "Failed to parse bad deeds as a positive integer")?;
        
        // Use Kid::new to create and return a Kid instance

        Ok(Self::new(name, good_deeds, bad_deeds))
    }

    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Self { name, niceness }
    }

    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }
}

pub fn main() -> io::Result<()> {
    // Specify the path to the CSV file
    let path = "5_day_kids.csv";

    // Open the file
    let file = File::open(&path).expect("Could not open the file");

    // Create a buffered reader for reading lines from the file
    let reader = io::BufReader::new(file);

    println!("Parsing Kids from CSV file:\n");

    // Process each line in the CSV
    for (index, line) in reader.lines().enumerate() {
        // The ? operator is shorthand for propagating errors.
        // If the Result is: Ok(value): It extracts the value (in this case, the String) and continues execution.
        // Err(error): It immediately returns the error from the current function and stops further execution.
        let line = line?;
        if let Ok(kid) = Kid::parse_row(&line) {
            println!("Row {}: {} is {}", index + 1, kid.name, kid.niceness);
        } else {
            // eprintln! is a macro used to print messages to the standard error stream
            eprintln!("Row {}: Failed to parse - {}", index + 1, line);
        }
    }

    Ok(())
}
