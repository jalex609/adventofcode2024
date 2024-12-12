use regex::{Captures, Regex};
use std::collections::HashSet;
fn main() {
    let contents = include_str!("../inputs/day3.txt");
    let regex = Regex::new(r"(?m)mul\((\d+),(\d+)\)").unwrap();
    let result = regex.captures_iter(&contents);

    let mut mult_vec : Vec<(usize, i32, i32)> = Vec::new();
    for multiply_cap in result {
        let index = find_index(&multiply_cap);
        let mul_num1 = &multiply_cap[1].parse::<i32>().unwrap();
        let mul_num2 = &multiply_cap[2].parse::<i32>().unwrap();
        mult_vec.push((index, *mul_num1, *mul_num2));
    }
    
    
    let do_indexes : Vec<(usize, &str)> = contents.match_indices("do()").collect();
    let dont_indexes : Vec<(usize, &str)> = contents.match_indices("don't()").collect();
    let mul_indexes: Vec<(usize, &str)> = mult_vec.iter().map(|(i, _, _)| {
        return (*i, "mul")
    }).collect();
    let mut final_indexes = [mul_indexes, do_indexes, dont_indexes].concat();
    final_indexes.sort_by(|(index_a, _), (index_b, _)| {
        return index_a.cmp(index_b);
    });

    let mut is_enabled = true;
    let mut is_enabled_index : HashSet<usize> = HashSet::new();

    for (index, opt_string) in final_indexes.into_iter() {
        is_enabled = match opt_string {
            "do()" => true,
            "don't()" => false,
            "mul" => is_enabled,
            _ => false
        };

        let is_mul = match opt_string {
            "mul" => true,
            _ => false
        };

        if is_mul && is_enabled {
            is_enabled_index.insert(index);
        }
    }

    let part_i = mult_vec.iter().fold(0, |acc, (_, num1, num2)| {
        return acc + (num1 * num2);
    });

    let part_ii = mult_vec.iter().fold(0, |acc, (index, num1, num2)| {
        let mut added = 0;
        if is_enabled_index.contains(index) {
            added = num1 * num2;  
        }
        return acc + added;
    });
    println!("Part I Solution: {}", part_i);
    println!("Part II Solution: {}", part_ii);

}

fn find_index(cap: &Captures) -> usize {
    // 0 is guaranteed to exist
    let overall_match = cap.iter().next().unwrap();
    let index  = match overall_match {
        Some(m) => return m.start(),
        None => 0
    };
    return index;
}