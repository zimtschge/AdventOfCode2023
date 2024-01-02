use std::fs;
use std::fmt;

struct EngineNumber{
    start_pos: usize,
    stop_pos: usize,
    number: i32,
}

impl fmt::Display for EngineNumber{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(Number {}, starts at {} and ends at {})", self.number, self.start_pos, self.stop_pos)
    }}

struct EngineSchematic{
    content : String
}

impl EngineSchematic{
    pub fn new(file: &str) -> Self {
        Self {content: fs::read_to_string(file).unwrap()}
    }

    pub fn get_row_count(&self) -> usize{
        self.content.lines().count()
    }

    pub fn get_row(&self, index: usize) -> String{
        let mut lines: std::str::Lines<'_> = self.content.lines();
        let mut line = String::new();
        for _ in 0..index+1{
            line = lines.next().unwrap().to_string();
        }
        return line;
    }

    pub fn get_numbers(&self, row: &str) -> Vec<EngineNumber>{
        let mut numbers: Vec<EngineNumber> = vec![];
        let mut number_str =  String::new();
        for (i, char) in row.chars().enumerate(){
            if char.is_digit(10){
                number_str.push(char)
            }
            else if number_str.len() != 0 {
                // If the string is filled, the prior characters were a number
                let number: EngineNumber = EngineNumber{
                    start_pos: i-number_str.len(), 
                    stop_pos: i-1, 
                    number: number_str.parse::<i32>().unwrap()};
                numbers.push(number);
                number_str.clear();
            }
        }
        numbers
    }

    pub fn check_for_neighbor(number: &EngineNumber, row: &String) -> bool{
        let start_check: usize = match number.start_pos{
            0 => 0,
            _ => number.start_pos - 1
        };
        
        let mut stop_check = number.stop_pos;
        if stop_check <= row.len(){
            stop_check = stop_check + 1;
        }

        for i in start_check..=stop_check{
            let char = row.as_bytes()[i];
            if (false == char.is_ascii_digit()) && (char != b'.'){
                return true;
            }
        }
        return false;
    }

    pub fn sum_all_parts(&self) -> i32{
        let mut sum : i32 = 0i32;
        for (i, line) in self.content.lines().enumerate(){
            for number in self.get_numbers(line){
                
                // Check in same row
                if true == EngineSchematic::check_for_neighbor(&number, &line.to_string()){
                    println!("In line {} the number {} has a neighbor in the same row", i, number.number);
                    sum = sum + number.number;
                    continue;
                }

                // Check in prior row
                let prior_row: Option<String> = match i {
                    0 => None,
                    _ => Some(self.get_row(i-1))
                };
                
                if prior_row.is_some(){
                    if true == EngineSchematic::check_for_neighbor(&number, &prior_row.unwrap()){
                        println!("In line {} the number {} has a neighbor in the prior row", i, number.number);
                        sum = sum + number.number;
                        continue;
                    }
                }
    
                // Check in next row
                let mut next_row: Option<String> = None;
                if i < self.get_row_count()-1{
                    next_row = Some(self.get_row(i+1));
                }

                if next_row.is_some(){
                    if true == EngineSchematic::check_for_neighbor(&number, &next_row.unwrap()){
                        println!("In line {} the number {} has a neighbor in the next row", i, number.number);
                        sum = sum + number.number;
                    }
                }
            }
        }
        sum
    }
}


fn main() {
    let engine: EngineSchematic = EngineSchematic::new("input.txt");
    println!("Solution of part 1: {}", engine.sum_all_parts());
}

const TEST_INPUT: &str = 
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";


#[cfg(test)]
mod tests {
    use crate::{TEST_INPUT, EngineSchematic};

    #[test]
    fn test_example(){
        assert_eq!(EngineSchematic{content: TEST_INPUT.to_string()}.sum_all_parts(), 4361);
    }
}