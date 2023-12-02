pub fn part_a(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let first_digit = line.chars().find(|c| c.is_digit(10));
            let last_digit = line.chars().rev().find(|c| c.is_digit(10));

            if let (Some(f), Some(l)) = (first_digit, last_digit) {
                f.to_digit(10).and_then(|fd| l.to_digit(10).map(|ld| fd * 10 + ld))
            } else {
                None
            }
        })
        .sum::<u32>() as i32
}

pub fn part_b(input: &str) -> i32 {
    let number_words = vec![
        ("one", '1'), ("two", '2'), ("three", '3'), 
        ("four", '4'), ("five", '5'), ("six", '6'), 
        ("seven", '7'), ("eight", '8'), ("nine", '9'),
    ];

    input
        .lines()
        .map(|line| {
            let mut matches = Vec::new();

            // Find all digit matches and convert them to char
            for mat in line.match_indices(char::is_numeric) {
                matches.push((mat.0, mat.1.chars().next().unwrap()));
            }

            // Check for number words at each character
            for (i, _) in line.char_indices() {
                for (word, digit) in &number_words {
                    if line[i..].starts_with(word) {
                        matches.push((i, *digit));
                    }
                }
            }

            // Sort the matches by index
            matches.sort_by_key(|&(index, _)| index);

            let first_digit = matches.first().map_or('0', |&(_, digit)| digit);
            let last_digit = matches.last().map_or(first_digit, |&(_, digit)| digit);

            first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap()
        })
        .sum::<u32>() as i32
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::{run_test_cases, TestCase};

    #[test]
    fn test_day_01_part_a() {
        println!("Current dir: {:?}", std::env::current_dir().unwrap());

        let test_cases = [
            
            TestCase::<i32>::new("1abc2", 12),
            TestCase::<i32>::new("pqr3stu8vwx", 38),
            TestCase::<i32>::new("a1b2c3d4e5f", 15),
            TestCase::<i32>::new("treb7uchet", 77),
            // File-based test case
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 142),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 56042)
        ];

        run_test_cases(part_a, &test_cases);
    }

    #[test]
    fn test_day_01_part_b() {
        let test_cases = [

            TestCase::<i32>::new("9963onefourthree6oneightq", 98),
            TestCase::<i32>::new("two1nine", 29),
            TestCase::<i32>::new("eightwothree", 83),
            TestCase::<i32>::new("abcone2threexyz", 13),
            TestCase::<i32>::new("xtwone3four", 24),
            TestCase::<i32>::new("4nineeightseven2", 42),
            TestCase::<i32>::new("zoneight234", 14),
            TestCase::<i32>::new("7pqrstsixteen", 76),
            // File-based test case
            TestCase::<i32>::new("file:inputs/part_b_1.txt", 281),
            TestCase::<i32>::new("file:inputs/part_b_2.txt", 55358)
        ];

        run_test_cases(part_b, &test_cases);
    }
}
