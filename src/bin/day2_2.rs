use std::io::{BufRead, BufReader, Result};
use std::collections::HashSet;
use std::fs::File;


fn main() -> std::io::Result<()> {
    // Get input from file
    let f = File::open("inputs/day2.txt")?;
    // Read the file linewise into a Vec. 
    // This returns a Result because there could be an error reading the file
    let words: Result<Vec<_>> = BufReader::new(f).lines().collect();
    // If there was an error, the ? operator will make the function 
    // return that error instead of continuing execution. 
    // If there wasn't an error it returns the wrapped value (in this case the lines of the file)
    let mut words = words?;
    
    // Iterate over every combination of two lines. 
    // 'outer is a loop label so I can break directly out of the outer loop
    'outer: for word1 in words.iter() {
        for word2 in words.iter() {
            // This block checks if are_close_enough returned a Some instead of a None
            // If it did return Some, it gets the string out of the some, 
            // assigns it to same, and then executes the block.
            // Otherwise it skips this block.
            if let Some(same) = are_close_enough(&word1, &word2) {
                println!("{}", same);
                break 'outer;
            }
        }
    }
    Ok(())
}

/// Checks if the two strings are the same except for one letter.
/// It returns an Option. If the Option is None then the two strings
/// aren't close enough. If the two strings are close enough 
/// it returns a Some containing the letters that are the same.
fn are_close_enough(x: &str, y: &str) -> Option<String> {
    let mut mismatches = 0;
    let mut same = String::new();
    // Goes through each char of the two strings
    for (c1, c2) in x.chars().zip(y.chars()) {
        if c1 == c2 {
            same.push(c1);
        } else {
            mismatches += 1;
        }
    }
    if mismatches == 1 {
        Some(same)
    } else {
        None
    }
}