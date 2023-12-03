advent_of_code::solution!(1);
use regex::Regex;



pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        // add up to sum
        let digits: String = line.chars().filter(|c| c.is_numeric()).collect();
        let first_char = digits.chars().next();
        let last_char = digits.chars().last();

        if let (Some(first), Some(last)) = (first_char, last_char) {
            let num_string: String = format!("{}{}", first, last);
            match num_string.parse::<u32>() {
                Ok(num) => sum += num,
                Err(_) => return None,
            };
        } else {
            return None;
        }
    }
    return Some(sum);
}
fn convert_written_number_to_u32(num: &str) -> u32 {
    
    let result:u32 = match num {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => {
            return num.parse::<u32>().unwrap()
        }
    };
    result
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    let pattern = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    
    for line in input.lines() {
        //let digits = pattern.find(input);
        let mut digits = pattern.find_iter(line);
        let first_char = match digits.next() {
            Some(first_char) => first_char,
            None => continue
        };
        //let last_char = digits.last();
        let last_char = match digits.last() {
            Some(last_char) => last_char,
            None => continue
        };
        let first = convert_written_number_to_u32(first_char.as_str());
        let last = convert_written_number_to_u32(last_char.as_str());
        let num_string: String = format!("{}{}", first, last);
            match num_string.parse::<u64>() {
                Ok(num) => sum += num,
                Err(_) => continue,
            };

        }
        return Some(sum);        
}
    




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY)).unwrap();

        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY)).unwrap();
        assert_eq!(result, 88);
    }

    
}
