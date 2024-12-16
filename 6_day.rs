/*
Day 6: Blitzen and Snowball’s Unproductive Debate
The North Pole Dev room was quiet—too quiet. Santa was still away and Blitzen was still in charge for the day, the elves didn't like that, some wished Santa's unhinged management style was back.

Blitzen leaned back in his reindeer chair knowing he's the boss now, sipping his coffee. “Hey, Snowball, did you know a function in Rust can return a reference?”

Snowball was a junior developer, he didn't know anything about Rust especially references, it seemed to confuse him, he looked up from their keyboard, skeptical. “That’s absurd. Functions return values, not references. You can’t return a borrowed value, Blitzen. It’ll dangle.”

“Oh, you poor, naive elf,” Blitzen said with a smug grin. “Behold the power of lifetimes!” He started scribbling on the whiteboard.

“Okay, but why do we even need this?” Snowball asked, raising an eyebrow. “What’s the use case?”

"We need to avoid unnecessary re-allocations, Snowball. It's more efficient this way. Remember day 2 when Santa was mad at us for a simple clone on a damn String? It wasn't even that big of a deal!"

“Fine! You're right, Santa hates clones.”

“I challenge you, Snowball. Write a function that returns the longer string without any re-allocation. Trim the strings, compare their lengths, and make sure it doesn't involve cloning or creating new allocations.”

The two bickered about ownership, lifetimes, and why Snowball wasn’t using Arch Linux for the next hour.

Now it’s your turn. Can you help Snowball write the function and put Blitzen in his place? Show that junior developers can handle lifetimes too! Try to finish the function longer_wish.

Requirements
If s1 is longer than s2, return a reference to s1 otherwise return a reference to s2 inside a Some variant.
If both slices have the same length, return None.
Any white spaces in the beginning or end of the string should be trimmed.
Good Luck!
*/

// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    // Trim both strings to remove leading and trailing whitespace
    // Count the number of Unicode characters in each string
    let len_s1 = s1.trim().chars().count();
    let len_s2 = s2.trim().chars().count();

    // Return a reference to the longer string (or None if they're equal in length)
    if len_s1 > len_s2 {
        Some(s1)
    } else if len_s2 > len_s1 {
        Some(s2)
    } else {
        None // If they are equal in length, return None
    }
}

fn main() {
    let wish1 = "  Merry Christmas!  ";
    let wish2 = "Happy Holidays!";

    match longer_wish(wish1, wish2) {
        Some(longer) => println!("The longer wish is: '{}'", longer),
        None => println!("Both wishes are of equal length."),
    }
}
