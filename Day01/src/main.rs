use std::fs;

fn main() {

    // Part One:
    let mut sum = 0i32;
    for line in fs::read_to_string("input.txt").unwrap().lines(){
        // Filter out all non digit characters
        let digits: String = line.chars().filter(|c| c.is_digit(10)).collect();
        
        // Get first digit with dealing with Option
        let mut first_digit: char = ' ';
        match digits.chars().next(){
            Some(c) => first_digit = c,
            None => println!("Could not extract first character")
        }

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
        let digits: String = result.chars().filter(|c| c.is_digit(10)).collect();

        let first_digit: char = digits.chars().next().unwrap();
        let last_digit: char = digits.chars().last().unwrap();

        let cal_val : String = first_digit.to_string() + &last_digit.to_string();
        sum2 = sum2 + cal_val.parse::<i32>().unwrap();
    }
    println!("Sum of part two: {}", sum2);
}
