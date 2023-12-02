pub fn part_a(input: &str) -> i32 {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    input
        .lines()
        .filter_map(|line| {
            if let Some((id_str, details)) = line.split_once(": ") {
                let game_id: i32 = id_str.split_whitespace().last()?.parse().ok()?;

                let is_possible = details.split("; ").all(|subset| {
                    subset.split(", ").all(|cube_info| {
                        if let Some((count_str, color)) = cube_info.rsplit_once(' ') {
                            if let Ok(count) = count_str.parse::<i32>() {
                                match color {
                                    "red" => count <= max_red,
                                    "green" => count <= max_green,
                                    "blue" => count <= max_blue,
                                    _ => false,
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                });

                if is_possible { Some(game_id) } else { None }
            } else {
                None
            }
        })
        .sum()
}


pub fn part_b(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            if let Some((_id_str, details)) = line.split_once(": ") {
                let mut min_red = 0;
                let mut min_green = 0;
                let mut min_blue = 0;

                for subset in details.split("; ") {
                    let (mut red, mut green, mut blue) = (0, 0, 0);
                    for cube_info in subset.split(", ") {
                        if let Some((count_str, color)) = cube_info.rsplit_once(' ') {
                            if let Ok(count) = count_str.parse::<i32>() {
                                match color {
                                    "red" => red = red.max(count),
                                    "green" => green = green.max(count),
                                    "blue" => blue = blue.max(count),
                                    _ => (),
                                }
                            }
                        }
                    }
                    min_red = min_red.max(red);
                    min_green = min_green.max(green);
                    min_blue = min_blue.max(blue);
                }

                min_red * min_green * min_blue
            } else {
                0
            }
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;
    use common::{run_test_cases, TestCase};

    #[test]
    fn test_day_02_part_a() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 8),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 2416),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_a, &test_cases);
    }

    #[test]
    fn test_day_02_part_b() {
        let test_cases = [
            TestCase::<i32>::new("file:inputs/part_a_1.txt", 2286),
            TestCase::<i32>::new("file:inputs/part_a_2.txt", 63307),
            // Add more test cases and the file-based test case here
        ];

        run_test_cases(part_b, &test_cases);
    }
}
