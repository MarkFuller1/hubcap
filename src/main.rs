/*
Tic Tac Toe in rust
 */
use std::io;

fn main() {
    // build board memory model
    let mut board: Vec<Vec<String>> = vec![vec![String::from("_"); 3]; 3];

    let mut next_move = String::from("x");

    while !is_complete(&board) {
        print_board(&board);

        println!("which column, which row");
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("failed to read line");

        let substrings: Vec<&str> = input.split(",").collect();
        let row: u32 = substrings[0].parse().unwrap();
        let column: u32 = substrings[1].lines().collect::<Vec<&str>>().join("").parse().unwrap();

        println!("{}: {}", row, column);

        make_move(&mut board, row, column, &next_move);

        if next_move == "x" {
            next_move = String::from("o");
        } else if next_move == "o" {
            next_move = String::from("x");
        }
    }

    println!("done")
}

fn make_move(board: &mut Vec<Vec<String>>, row: u32, col: u32, value: &String) {
    board[row as usize][col as usize] = value.clone();
}

fn print_board(board: &Vec<Vec<String>>) {
    for row in board {
        for column in row {
            print!("{}", column);
        }
        print!("\r\n")
    }
}

fn is_complete(board: &Vec<Vec<String>>) -> bool {
    let win_masks: Vec<u32> = vec![448, 292, 146, 73, 56, 7, 273, 84];
    let mask: String = flatten(board).concat();

    let x_win_mask: String = create_win_mask(&mask, 'x');
    let o_win_mask: String = create_win_mask(&mask, 'o');

    if win_masks.contains(&u32::from_str_radix(x_win_mask.as_str(), 2).unwrap())
    {
        println!("x won!");
        print_board(&board);
        return true;
    }

    if win_masks.contains(&u32::from_str_radix(o_win_mask.as_str(), 2).unwrap())
    {
        println!("o won!");
        print_board(&board);
        return true;
    }
    return false;
}

fn flatten(nested: &Vec<Vec<String>>) -> Vec<String> {
    let mut flat: Vec<String> = vec![];
    nested.iter().for_each(|row| {
        row.iter().for_each(|item| {
            flat.push(item.clone());
        })
    });

    flat
}

fn create_win_mask(array: &String, winner: char) -> String {
    array.chars().into_iter().map(|x| {
        if x == winner {
            return "1";
        }
        return "0";
    }).collect()
}
