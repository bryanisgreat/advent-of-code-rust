advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut input_iter = input.split("\n\n");

    let fresh_ranges: Vec<(u64, u64)> = input_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let fresh_item_count = input_iter
        .next()
        .unwrap()
        .lines()
        .filter(|item| {
            let id: u64 = item.parse().unwrap();
            fresh_ranges.iter().any(|r| id >= r.0 && id <= r.1)
        })
        .count() as u64;

    Some(fresh_item_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input_iter = input.split("\n\n");

    let mut ranges: Vec<(u64, u64)> = input_iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once('-').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    let mut to_remove = Vec::new();
    let mut to_add = Vec::new();

    loop {
        for (i, id_range) in ranges.iter().enumerate() {
            for (j, cross_range) in ranges.iter().enumerate() {
                if i == j { continue }
                let start_in_range = id_range.0 >= cross_range.0 && id_range.0 <= cross_range.1;
                let end_in_range = id_range.1 <= cross_range.1 && id_range.1 >= cross_range.0;
                if start_in_range || end_in_range {
                    let updated_range = (cross_range.0.min(id_range.0), cross_range.1.max(id_range.1));
                    if !to_remove.contains(&i) { to_remove.push(i) }
                    if !to_add.contains(&updated_range) { to_add.push(updated_range) }
                }
            }
        }
        if to_remove.is_empty() && to_add.is_empty() { break }
        to_remove.sort_unstable();
        for i in to_remove.iter().rev() {
            ranges.remove(*i);
        }
        ranges.extend(to_add.drain(..));
        to_remove.clear();
    }
        
    let total = ranges.iter().map(|i| i.1 - i.0 +1).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
