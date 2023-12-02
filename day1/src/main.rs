fn find_first(s: &str, numbers: &Vec<(&str, i32)>) -> Option<i32> {
    let mut s = s.to_string();

    while !s.is_empty() {
        for &(word_num, num) in numbers {
            if s.starts_with(word_num) {
                return Some(num);
            }
        }

        if let Some(first_char) = s.chars().next() {
            if let Some(digit) = first_char.to_digit(10) {
                return Some(digit as i32);
            }
        }

        s.remove(0);
    }
    None
}

fn sum_of_two_integers(s: &str) -> Option<i32> {
    let numbers: Vec<(&str, i32)> = vec![("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    let first = find_first(s, &numbers)?;

    let reversed_s: String = s.chars().rev().collect();
    let reversed_numbers = vec![("orez", 0), ("eno", 1), ("owt", 2), ("eerht", 3), ("ruof", 4), ("evif", 5), ("xis", 6), ("neves", 7), ("thgie", 8), ("enin", 9)];

    let last = find_first(&reversed_s, &reversed_numbers)?;

    Some(first*10 + last)
}

fn main() {
    let content = std::fs::read_to_string("question.txt").unwrap();
    let total: i32 = content.lines().filter_map(sum_of_two_integers).sum();
    println!("Total: {}", total);
}
