
fn main () {
    let contents = include_str!("../inputs/day2.txt");
    let report_list = parse_input(contents);

    let mut safe_report_count_part1 = 0;
    let mut safe_report_count_part2 = 0;

    for report in report_list.iter() {
        let is_safe = is_safe_report(report); 
        if is_safe {
            safe_report_count_part1 += 1;
            safe_report_count_part2 += 1;
        } else {
            for (i, _) in report.iter().enumerate() {
                // Get rid of 1 level at a time and enumerate through
                let report_minus_1 : Vec<i32> = report.iter().enumerate().filter_map(|(index, level)|  {
                    if index == i {
                        return None
                    } else {
                        return Some(*level)
                    }
                }).collect();
                let is_subset_safe = is_safe_report(&report_minus_1);
                if is_subset_safe {
                    safe_report_count_part2 += 1;
                    break;
                }
            }
        }
    }

    println!("Safe report count part 1={}", safe_report_count_part1);
    println!("Safe report count part 2={}", safe_report_count_part2);
}

fn is_safe_report(report : &Vec<i32>) -> bool {
    let is_asc_or_desc = report.iter().rev().is_sorted() || 
    report.iter().is_sorted();
    let diff_is_within = is_difference_within_value(1, 3, report);

    return is_asc_or_desc && diff_is_within;
}

fn is_difference_within_value(diff_min_value: i32, diff_max_value: i32, report: &Vec<i32>) -> bool {
    for (i, level) in report.iter().enumerate() {

        let mut before_diff = diff_min_value;
        if i != 0 {
            before_diff = match report.get(i.saturating_sub(1)) {
                Some(num) => (level - num).abs(),
                None => diff_min_value
            };
        }
        let mut after_diff = diff_min_value;
        if i != report.len() - 1 {
            after_diff = match report.get(i.saturating_add(1)) {
                Some(num) => (level - num).abs(),
                None => diff_min_value
            };
        }

        let is_within = (before_diff >= diff_min_value && before_diff <= diff_max_value) &&
         (after_diff >= diff_min_value && after_diff <= diff_max_value);
         if is_within == false {
            return is_within
         }
    }
    return true;
}

fn parse_input(contents : &str) -> Vec<Vec<i32>> {
    let lines = contents.lines();
    let mut report_list: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let mut report_arr: Vec<i32> = Vec::new();
        let mut levels = line.split_whitespace();
        let mut next_level = levels.next();
        while next_level != None {
            let level_val = next_level.unwrap();
            report_arr.push(level_val.parse::<i32>().unwrap());
            next_level = levels.next()
        }
        report_list.push(report_arr);
    }

    return report_list;
}