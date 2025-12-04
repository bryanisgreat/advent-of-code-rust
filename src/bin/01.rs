advent_of_code::solution!(1);

fn parse_inputs(input: &str) -> (i32, i32) {
    let (dir, cnt) = input.split_at(1);
    let count: i32 = cnt.parse().unwrap();
    let direction = match dir {
        "L" => -1,
        "R" => 1,
        _ => panic!("unexpected char"),
    };
    (direction, count)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut code_number = 0;

    for line in input.lines()  {
        let (direction, count) = parse_inputs(line);

        let new_number = position + (direction * count % 100);
        position = match new_number {
            x if x < 0 => 100 + x,
            x if x > 99 => x - 100,
            _ => new_number
        };
        if position == 0 {
            code_number += 1;
        }
    }

    Some(code_number)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut position = 50;
    let mut code_number: u64 = 0;

    for line in input.lines()  {
        let (direction, count) = parse_inputs(line);

        let new_number = position + (direction * count % 100);
        position = match new_number {
            0 => 0,
            100 => 0,
            x if x < 0 => {
                if position != 0 {code_number += 1}
                100 + x
            }
            x if x > 99 => {
                code_number += 1;
                x - 100
            }
            _ => new_number
        };
        if position == 0 {
            code_number += 1;
        }
        code_number += (count /100).abs() as u64;
    }

    Some(code_number)
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
        assert_eq!(result, Some(6));
    }
}
