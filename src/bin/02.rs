advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.split(',');
    let mut total: u64 = 0;

    for range in ranges {
        let mut split = range.trim().split('-');
        let start: u64 = split.next().unwrap().parse().expect(&format!("THISFAILED YOU IDIOT start {}", range));
        let end: u64 = split.last().unwrap().parse().expect(&format!("THISFAILED YOU IDIOT end {}", range));

        for record in start..=end {
            let record = record.to_string();
            let split_size = record.len() / 2;

            let split_record = record.split_at(split_size);
            if split_record.0 == split_record.1 {
                total += record.parse::<u64>().unwrap();
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split(',');
    let mut total: u64 = 0;

    for range in ranges {
        let mut split = range.trim().split('-');
        let start: u64 = split.next().unwrap().parse().expect(&format!("THISFAILED YOU IDIOT start {}", range));
        let end: u64 = split.last().unwrap().parse().expect(&format!("THISFAILED YOU IDIOT end {}", range));

        for record in start..=end {
            let record = record.to_string();
            let max_size = record.len() / 2;

            'search: for search_size in 1..=max_size {
                let record_chars: Vec<char> = record.chars().collect();
                let first_chunk: String = record_chars.chunks(search_size).next().unwrap().iter().collect();
                for split_part in record_chars.chunks(search_size) {
                    if first_chunk != split_part.iter().collect::<String>() {
                        continue 'search
                    }
                }
                total += record.parse::<u64>().unwrap();
                break 'search
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
