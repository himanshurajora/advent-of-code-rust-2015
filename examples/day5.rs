use std::collections::{HashMap, HashSet};

struct StringStats {
    input: String,
}

impl StringStats {
    fn new(input: String) -> StringStats {
        StringStats { input }
    }

    fn is_nice_string(&self) -> bool {
        let mut vovels: Vec<char> = vec![];
        let mut has_repeating_char = false;
        let mut has_unwanted_sequence = false;
        let mut previous: char = '0';
        for current in self.input.chars() {
            if is_vowel(current) {
                vovels.push(current);
            }

            if current == previous {
                has_repeating_char = true;
            }

            if is_bad_sequence(current, previous) {
                println!("Unwatned sequence condition previous {previous} current {current}");
                has_unwanted_sequence = true;
            }

            previous = current;
        }

        println!(
            "input {}, stats: vovels {:?}, repeating {} unwanted sequence {}",
            self.input, vovels, has_repeating_char, has_unwanted_sequence
        );
        // checks if string is nice
        vovels.len() > 2 && has_repeating_char == true && has_unwanted_sequence == false
    }

    fn is_nice_string_2(&self) -> bool {
        let mut vovels: Vec<char> = vec![];
        let mut previous: char = '0';
        let mut next: char = '0';
        // It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
        let mut condition_one = false;
        // It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
        let mut condition_two = false;

        let mut pairs: HashMap<String, usize> = HashMap::new();

        let mut previous_pair = String::from("");

        for (index, current) in self.input.chars().enumerate() {
            let next = self.input.chars().nth(index + 1);
            let current_pair = format!("{}{}", current, next.unwrap_or('0'));

            if current_pair == previous_pair {
                // println!("Condition made false {}, {}", previous_pair, current_pair);
                return false;
            }

            // println!("{}, {}", previous_pair, current_pair);
            previous_pair = current_pair.clone();

            if let Some(existing) = pairs.get(current_pair.as_str()) {
                // println!("existing {}, {}", current_pair, existing);
                pairs.insert(current_pair, existing + 1);
                condition_one = true;
            } else {
                pairs.insert(current_pair, 1);
            }

            if is_vowel(current) {
                vovels.push(current);
            }

            if let Some(value) = next {
                if previous == value {
                    // println!("prev {}, current {}", previous, value);
                    condition_two = true;
                }
            }

            previous = current;

        }

        // println!("condition 1 {}, condition 2 {}", condition_one, condition_two); 

        condition_one && condition_two
    }
}

fn is_vowel(input: char) -> bool {
    // we can also create an array and match them using includes
    // I know that just not going to do that
    input == 'a' || input == 'e' || input == 'i' || input == 'o' || input == 'u'
}

fn is_bad_sequence(current: char, previous: char) -> bool {
    (previous == 'a' && current == 'b')
        || (previous == 'c' && current == 'd')
        || (previous == 'p' && current == 'q')
        || (previous == 'x' && current == 'y')
}

struct App {
    strings: Vec<StringStats>,
}

impl App {
    fn new(strings: &str) -> App {
        App {
            strings: strings
                .lines()
                .map(|line| StringStats::new(line.to_string()))
                .collect(),
        }
    }

    fn get_number_of_nice_strings(&mut self) -> u32 {
        self.strings.iter().fold(0, |sum, stat| {
            if stat.is_nice_string() {
                return sum + 1;
            }
            sum
        })
    }

    fn get_number_of_nice_strings_2(&mut self) -> u32 {
        self.strings.iter().fold(0, |sum, stat| {
            if stat.is_nice_string_2() {
                return sum + 1;
            }
            sum
        })
    }
}

fn main() {
    println!(
        "{}",
        App::new(include_str!("../data/day5.txt")).get_number_of_nice_strings()
    );
    println!(
        "{}",
        App::new(include_str!("../data/day5.txt")).get_number_of_nice_strings_2()
    );
}

#[cfg(test)]
pub mod test {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            StringStats::new("ugknbfddgicrmopn".to_string()).is_nice_string(),
            true
        );

        assert_eq!(StringStats::new("aaa".to_string()).is_nice_string(), true);
        assert_eq!(
            StringStats::new("jchzalrnumimnmhp".to_string()).is_nice_string(),
            false
        );
        assert_eq!(
            StringStats::new("haegwjzuvuyypxyu".to_string()).is_nice_string(),
            false
        );
        assert_eq!(
            StringStats::new("dvszwmarrgswjxmb".to_string()).is_nice_string(),
            false
        );
    }


    #[test]
    fn test_2() {
        assert_eq!(StringStats::new("qjhvhtzxzqqjkmpb".to_string()).is_nice_string_2(), true);
        assert_eq!(StringStats::new("xxyxx".to_string()).is_nice_string_2(), true);
        assert_eq!(StringStats::new("uurcxstgmygtbstg".to_string()).is_nice_string_2(), false);
        assert_eq!(StringStats::new("ieodomkazucvgmuy".to_string()).is_nice_string_2(), false);
    }
}
