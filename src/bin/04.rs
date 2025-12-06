advent_of_code::solution!(4);



fn scan(
    area: &Vec<Vec<u64>>,
    top: usize,
    scan_row: usize,
    bottom: usize,
) -> Option<u64> {
    let mut total: u64 = 0;
    let width = area[0].len();
    for i in 0..width {
        let check_cell = area[scan_row].get(i).unwrap_or(&0);
        if check_cell == &0 { continue }

        let mut roll_count: u64 = 0;
        if i > 0 {
            roll_count += area[top].get(i-1).unwrap_or(&0);
            roll_count += area[scan_row].get(i-1).unwrap_or(&0);
            roll_count += area[bottom].get(i-1).unwrap_or(&0);
        }

        roll_count += area[top].get(i).unwrap_or(&0);
        roll_count += area[top].get(i+1).unwrap_or(&0);

        roll_count += area[scan_row].get(i+1).unwrap_or(&0);
        roll_count += area[bottom].get(i).unwrap_or(&0);
        roll_count += area[bottom].get(i+1).unwrap_or(&0);

        if roll_count < 4 {
            total += 1;
        }
    }
    // println!("subtotal {total}");
    // println!("{:?}", area[top]);
    // println!("{:?}", area[scan_row]);
    // println!("{:?}", area[bottom]);
    // println!();

    Some(total)
}

pub fn part_one(input: &str) -> Option<u64> {
    _ = input;

    // read input line and mark possible items positions
    // line one would be everything
    // line 2 would decide line 1 outcome
    // line 3 would decide line 2 outcome along with line 1
    // repeat like this
    // line N would decide line N and N -1
    //
    //
    //  need 3 buffers


    let mut total: u64 = 0;
    let lines = input.trim().lines();
    let width = lines.clone().peekable().peek().unwrap().len(); // clone hack?
    let line_count = lines.clone().count();

    let mut area = vec![vec![0; width]; 3];
    let mut current_row = 0;
    let mut scan_row: i32 = 0;

    for (li, line) in lines.enumerate() {
        for (i, ch) in line.chars().enumerate() {
            area[current_row][i] = match ch {
                '@' => 1,
                '.' => 0,
                _ => panic!("unknown thingy")
            }
        }
        current_row = (current_row + 1).rem_euclid(3);

        // Skip incomplete data
        if li == 0 { continue }

        // First Scan
        if li == 1 {
            let top = 2;
            let bottom = 1;
            let mid = scan_row as usize;
            total += scan(&area, top, mid, bottom).unwrap();
            scan_row = 1;
            continue
        }

        // All the middle scans
        let top: usize = (scan_row - 1).rem_euclid(3) as usize;
        let bottom: usize = (scan_row + 1).rem_euclid(3) as usize;
        let mid = scan_row as usize;
        total += scan(&area, top, mid, bottom).unwrap();
        scan_row = (scan_row + 1).rem_euclid(3);


        if ( li + 1 ) < line_count { continue }

        // Last Scan
        let top: usize = (scan_row - 1).rem_euclid(3) as usize;
        let bottom: usize = (scan_row + 1).rem_euclid(3) as usize;
        let mid = scan_row as usize;
        area[bottom].fill(0);
        total += scan(&area, top, mid, bottom).unwrap();
    }
        
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let data: Vec<Vec<i32>> = input
        .trim()
        .lines()
        .map(|line|
             line
             .chars()
             .map(|ch|
                  match ch {
                      '@' => 1,
                      _ => 0,
                  }
             )
             .collect())
        .collect();

    let mut total: u64 = 0;
    let (mut result_count, mut result) = count_removals(&data);
    total += result_count;

    loop {
        (result_count, result) = count_removals(&result);
        total += result_count;
        if result_count == 0 { break }
    }
    Some(total)
}

fn count_removals(data: &Vec<Vec<i32>>) -> (u64, Vec<Vec<i32>>) {
    
    let mut result: Vec<Vec<i32>> = data.clone();
    let mut subtotal: u64 = 0;

    for (irow, _row) in data.iter().enumerate() {
        let target = &data[irow];
        let adjacents: Vec<&Vec<i32>> = match irow {
            0 =>  vec![&data[irow], &data[irow+1]],
            i if i == data.len() - 1 => vec![&data[irow], &data[irow-1]],
            _ => vec![&data[irow], &data[irow-1], &data[irow+1]],
        };

        for (icol, col) in target.iter().enumerate() {
            let mut adjacent_count = -1;
            if *col == 1 {
                for adjacent in &adjacents {
                    for (iadjacent_col, adjacent_col) in adjacent.iter().enumerate(){
                        let is_really_adjacent = iadjacent_col == icol || iadjacent_col + 1 == icol || (iadjacent_col > 0 && iadjacent_col - 1 == icol);
                        if is_really_adjacent && *adjacent_col == 1 {
                           // we have a match!
                           adjacent_count += 1;
                        }
                    }
                }

                if adjacent_count < 4 {
                    // we can remove this one
                    result[irow][icol] = adjacent_count * -1;
                    subtotal += 1;
                }
            }
        }
    }

    (subtotal, result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
