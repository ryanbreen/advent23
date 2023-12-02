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


pub fn part_b(_: &str) -> i32 {
    // Your solution for part B
    return 2;
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
            TestCase::<i32>::new("file:inputs/day_01_part_a_1.txt", 142),
            TestCase::<i32>::new("file:inputs/day_01_part_a_2.txt", 56042)
        ];

        run_test_cases(part_a, &test_cases);
    }

    #[test]
    fn test_day_01_part_b() {
        let test_cases = [
        ];

        run_test_cases(part_b, &test_cases);
    }
}
