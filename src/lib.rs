use rand::Rng;
use std::ops::Range;
use rand::distributions::uniform::{SampleUniform};

/// Generate a random number within a range
///
/// # Arguments
///
/// * `range` - The range of numbers to generate from
///
/// # Example
///
/// ```
/// use rust_go_fish::get_random;
///
/// let num = get_random(0..10);
/// assert!(num >= 0 && num < 10);
/// ```
pub fn get_random<T>(range: Range<T>) -> T
    where
        T: PartialOrd + Copy + SampleUniform,
{
    if range.start >= range.end {
        // return None; // Return None for an empty or invalid range
        return range.start;
    }

    let mut rng = rand::thread_rng(); // Create a random number generator
    rng.gen_range(range.start..range.end) // Generate a random number
}


/// Generate a random number within a range, excluding a specific number
///
/// # Arguments
///
/// * `range` - The range of numbers to generate from
/// * `exclude` - The number to exclude
///
/// # Example
///
/// ```
/// use rust_go_fish::get_random_excluding;
///
/// let num = get_random_excluding(0..10, 5);
/// assert_ne!(num, 5);
/// ```
pub fn get_random_excluding<T>(range: Range<T>, exclude: T) -> T
    where
        T: PartialOrd + Copy + SampleUniform,
{
    let mut rng = rand::thread_rng(); // Create a random number generator

    loop {
        let num = rng.gen_range(range.start..range.end); // Generate a random number
        if num != exclude {
            return num;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random() {
        let num = get_random(0..10);
        assert!(num >= 0 && num < 10);
    }

    #[test]
    fn test_get_random_with_empty_range() {
        let num = get_random(0..0);
        assert!(num >= 0 && num < 10);
    }

    #[test]
    fn test_get_random_excluding() {
        let num = get_random_excluding(0..10, 5);
        assert_ne!(num, 5);
    }
}