advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.trim().split('\n');
    let mut total: u64 = 0;
    for line in lines {
        let mut first_digit: u64 = 0;
        let mut second_digit: u64 = 0;
        let mut first_pos = 0;

        for (i, ch) in line.chars().enumerate() {
            if i+1 == line.len() {break}

            let digit: u64 = ch.to_string().parse().unwrap();
            if digit > first_digit {
                first_digit = digit;
                first_pos = i;
            }
        }

        for ch in line.chars().skip(first_pos+1) {
            let digit: u64 = ch.to_string().parse().unwrap();
            if digit > second_digit {
                second_digit = digit;
            }
        }

        total += format!("{}{}", first_digit, second_digit).parse::<u64>().unwrap();
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.trim().split('\n');
    let mut total: u64 = 0;
    for line in lines {
        let mut digits = [0; 12];
        let mut last_used_position = 0;
        let joltage_size = digits.len();

        for current_digit in 0..joltage_size {
            let skip_items = last_used_position;
            let remaining_digits_to_set = joltage_size - current_digit;
            let take_items = 1 + line.len() - skip_items - remaining_digits_to_set;

            for (i, ch) in line.chars().skip(skip_items).take(take_items).enumerate() {
                let digit: u64 = ch.to_string().parse().unwrap();

                if digit > digits[current_digit] {
                    digits[current_digit] = digit;
                    last_used_position = skip_items + i + 1;
                }
            }
        }

        let numberic: u64 = digits.map(|i| i.to_string()).join("").parse().expect("Supposed to be a number...");
        total += numberic;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
