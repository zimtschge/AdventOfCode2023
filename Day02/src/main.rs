use std::fs;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let mut sum_of_failed_games : i32 = 0;
    let mut max_game: i32 = 0;

    for line in fs::read_to_string("input.txt").unwrap().lines(){
        // 1. Extract game number
        let (game_no, game_str) = line.split_once(':').unwrap();
        let game_no = &game_no[5..];
        let game_no: i32 = game_no.parse::<i32>().unwrap();
        max_game = game_no;

        // 2. Separate each reveal by ';'
        'sets: for sets in game_str.split(';')
        {
            for set in sets.split(',')
            {
                let (cnt, color) = set[1..].split_once(' ').unwrap();
                let cnt = cnt.parse::<i32>().unwrap();
                let max_available : i32 = match color {
                    "green" => MAX_GREEN,
                    "red" => MAX_RED,
                    "blue" => MAX_BLUE,
                    _ => panic!("Oh no color")
                };

                if cnt > max_available
                {
                    sum_of_failed_games = sum_of_failed_games + game_no;
                    break 'sets
                }
            }
        }
    }
    //let sum_100 = (0..(max_game+1)).fold(0, |a, b| a + b);
    let sum_100: i32 = (0..(max_game+1)).sum();
    let result_a: i32 = sum_100 - sum_of_failed_games;
    println!("Solution of part a: {}", result_a);
}
