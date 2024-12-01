fn main() {
    let input: String = common::AocInput::fetch(2015, 5).unwrap().into();
    println!("Output: {}", process(&input));
}

fn process(input: &str) -> usize {
    input.lines().filter(|l| is_nice(l)).count()
}

fn is_nice(line: &str) -> bool {
    let mut prev = None;
    let mut vowel_count = 0;
    let mut doubles = false;
    for char in line.chars() {
        if matches!(char, 'a' | 'e' | 'i' | 'o' | 'u') {
            vowel_count += 1
        }
        if let Some(prev) = prev {
            let combined = format!("{prev}{char}");
            if matches!(combined.as_str(), "ab" | "cd" | "pq" | "xy") {
                return false;
            }
            if prev == char {
                doubles = true;
            }
        }
        prev = Some(char);
    }

    vowel_count >= 3 && doubles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(is_nice("ugknbfddgicrmopn"));
        assert!(is_nice("aaa"));
        assert!(!is_nice("jchzamnmhp"));
        assert!(!is_nice("haegwjzuvuyypxyu"));
        assert!(!is_nice("dvszwmarrgswjxmb"));
    }
}
