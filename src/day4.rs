use std::collections::HashSet;

fn main() {
    let contents = include_str!("../inputs/day4.txt");
    let rows: Vec<Vec<char>> = contents
        .lines()
        .into_iter()
        .map(|line_str| {
            return line_str.chars().collect()
        })
        .collect();

    let mut num_xmas = 0;
    for (i, _) in rows.iter().enumerate() {
        for (j, _) in rows[i].iter().enumerate() {
            let current_letter = rows[i][j];
            if current_letter == 'X' {
                let num_occurences = find_neighbor_string("MAS", vec![((i, j), None, &'M')], &rows);
                num_xmas += num_occurences;
            }
        }
    }
    println!("Solution for part I: {num_xmas}");
}

fn find_neighbor_string(
    to_find: &str,
    current_posns: Vec<((usize, usize), Option<&str>, &char)>,
    puzzle: &Vec<Vec<char>>,
) -> i32 {
    let mut num_occurences = 0;
    let mut posns = current_posns;
    let mut final_posns: HashSet<(i32, i32)> = HashSet::new();
    let characters : Vec<char> = to_find.chars().collect();

    while posns.len() > 0 {
        let posn = posns.pop().unwrap();
        let (xy_coords, curr_direction, char_to_find) = posn;
        let neighbors = find_neighbors(xy_coords, &puzzle);
        for ((row, column), direction) in neighbors {
            let neighbor_str = puzzle[row as usize][column as usize];
            if neighbor_str == *char_to_find && (curr_direction == None || curr_direction == Some(direction)) {
                if char_to_find == characters.last().unwrap() {
                    if final_posns.contains(&(row, column)) == false {
                        num_occurences += 1;
                    }
                    final_posns.insert((row, column));
                } else {
                    posns.push(((row as usize, column as usize), Some(direction), get_next_letter(&characters, *char_to_find).unwrap()));
                }
            }
        }   
    }
    return num_occurences;
}

fn get_next_letter(full_str : &Vec<char>, current_char: char) -> Option<&char> {
    let current_idx = full_str.iter().position(|&x| x == current_char).unwrap();
    return full_str.get(current_idx + 1);
}

fn find_neighbors<A>(pos: (usize, usize), two_d_array: &Vec<Vec<A>>) -> HashSet<((i32, i32), &str)> {
    let num_columns = two_d_array.iter().len() as i32;
    let num_rows = two_d_array.len() as i32;
    let (row, column) = pos;
    let row_int = row as i32;
    let column_int = column as i32;
    // set up posn's
    let above_row = row_int - 1;
    let down_row = row_int + 1;
    let left_column = column_int - 1;
    let right_column = column_int + 1;

    // create posns
    let above = (above_row, column_int);
    let left = (row_int, left_column);
    let right = (row_int, right_column);
    let down = (down_row, column_int);
    let diag_up_left = (above_row, left_column);
    let diag_up_right = (above_row, right_column);
    let diag_down_left = (down_row, left_column);
    let diag_down_right = (down_row, right_column);

    // remove   
    let posns: HashSet<((i32, i32), &str)> = [
        (above, "above"),
        (left, "left"),
        (right, "right"),
        (down, "down"),
        (diag_up_left, "diag_up_left"),
        (diag_up_right, "diag_up_right"),
        (diag_down_left, "diag_down_left"),
        (diag_down_right, "diag_down_right")
    ]
    .into_iter()
    .filter(|((row, column), _)| {
        return !(*row < 0 || *column < 0 || *row >= num_rows || *column >= num_columns);
    })
    .collect();
    return posns;
}
