use std::fs;

fn filter_digits_only(input: &str) -> String
{
    return input.chars().filter(|c| c.is_digit(10)).collect();
}

fn extract_first_digit(input: &String) -> char{
    match input.chars().next(){
        Some(c) => return c,
        None => panic!("Show how a Option match statement can work")
    }
}

fn main() {

    // Part One:
    let mut sum = 0i32;
    for line in fs::read_to_string("input.txt").unwrap().lines(){
        // Filter out all non digit characters
        let digits: String = filter_digits_only(line);
        
        // Get first digit with dealing with Option
        let first_digit: char = extract_first_digit(&digits);

        // Get last digit using unwrap()
        let last_digit: char = digits.chars().last().unwrap();

        let cal_val : String = first_digit.to_string() + &last_digit.to_string();
        sum = sum + cal_val.parse::<i32>().unwrap();
    }
    println!("Sum of part one: {}", sum);

    // Part Two:
    // Not working since Reddit says oneight is considered to be 18 ^^
    let mut sum2 = 0i32;
    for line in fs::read_to_string("input.txt").unwrap().lines(){
        let mut result = line.replace("one", "1one");
        result = result.replace("two", "2two");
        result = result.replace("three", "3three");
        result = result.replace("four", "4four");
        result = result.replace("five", "5five");
        result = result.replace("six", "6six");
        result = result.replace("seven", "7seven");
        result = result.replace("eight", "8eight");
        result = result.replace("nine", "9nine");

        // Filter out all non digit characters
        let digits: String = filter_digits_only(&result);

        let first_digit: char = digits.chars().next().unwrap();
        let last_digit: char = digits.chars().last().unwrap();

        let cal_val : String = first_digit.to_string() + &last_digit.to_string();
        sum2 = sum2 + cal_val.parse::<i32>().unwrap();
    }
    println!("Sum of part two: {}", sum2);
}
