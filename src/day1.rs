use std::fs;
use std::str;
use std::vec::IntoIter;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("../inputs/day1.txt").unwrap();
    let (location_id_left_iter, location_id_right_iter) = parse_input(&contents);
    // Sort in place
    let mut location_ids_left: Vec<i32> = location_id_left_iter.collect();
    let mut location_ids_right: Vec<i32> = location_id_right_iter.collect();
    location_ids_left.sort();
    location_ids_right.sort();

    println!("Final output part 1: {}", parse_total_distance(&location_ids_left, &location_ids_right));
    println!("Final output part 2: {}", parse_total_similarity(&location_ids_left, &location_ids_right));

}

fn parse_input(contents : &str) -> (IntoIter<i32>, IntoIter<i32>) {
    let lines = contents.lines();
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();

    for line in lines {
        let location_ids: std::str::Split<'_, &str> = line.split("   ");
        let collection: Vec<&str> = location_ids.collect();

        vec1.push(collection.get(0).unwrap().parse::<i32>().unwrap());
        vec2.push(collection.get(1).unwrap().parse::<i32>().unwrap());
    }

    return (vec1.into_iter(), vec2.into_iter());
}

fn parse_total_distance(location_ids_left : &Vec<i32>, location_ids_right: &Vec<i32>) -> i32 {
    let mut final_distance_sum : i32 = 0;
    for n in 0..location_ids_left.len() {
        let location_id_left = location_ids_left.get(n).unwrap();
        let location_id_right = location_ids_right.get(n).unwrap();

        let final_distance = (location_id_left - location_id_right).abs();
        final_distance_sum += final_distance;
    }

    return final_distance_sum;
}

fn parse_total_similarity(location_ids_left : &Vec<i32>, location_ids_right: &Vec<i32>) -> i32 {
    let mut number_of_occurences = HashMap::<i32, i32>::new();
    for location_id in location_ids_right {
        if number_of_occurences.contains_key(&location_id) {
            number_of_occurences.insert(*location_id, number_of_occurences[&location_id] + 1);
        } else {
            number_of_occurences.insert(*location_id, 1);
        }
    }

    let mut final_similarity_score : i32 = 0;
    for n in 0..location_ids_left.len() {
        let location_id_left = location_ids_left.get(n).unwrap();
        let num_occurences_of_left = match number_of_occurences.get(location_id_left) {
            Some(final_num_occurences) => *final_num_occurences,
            None => 0
        };

        let final_similarity = location_id_left * num_occurences_of_left;
        final_similarity_score += final_similarity;
    }

    return final_similarity_score;
}