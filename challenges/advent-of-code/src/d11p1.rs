use std::fs::File;
use std::io::prelude::*;
use std::env::args;

#[derive(Debug,PartialEq,Clone)]
enum LocationState {
    EmptySeat,
    FilledSeat,
    Floor,
}

fn print_state(all_seats: &Vec<LocationState>, row_length: usize) {
    let mut buffer = String::new();
    let mut cur_row = 0;
    for (i, pos) in all_seats.iter().enumerate() {
        let pos_row = i / row_length;
        if pos_row > cur_row {
            cur_row = pos_row;
            buffer.push('\n');
        }
        buffer.push(match pos {
            LocationState::EmptySeat => 'L',
            LocationState::FilledSeat => '#',
            LocationState::Floor => '.',
        });
    }
    println!("{}", buffer);
}

fn count_seats_in_use(all_seats: &Vec<LocationState>) -> usize {
    all_seats
        .iter()
        .filter(|s| s == &&LocationState::FilledSeat)
        .count()
}

fn count_seats_in_use_around(all_seats: &Vec<LocationState>, row_length: usize, seat: usize) -> usize {
    let mut in_use_around = 0;

    let index_in_use = |i| -> bool {
        match all_seats.get(i) {
            None => false,
            Some(seat) => match seat {
                LocationState::FilledSeat => true,
                _ => false,
            },
        }
    };

    let is_top_row = seat < row_length;
    let is_bottom_row = seat > all_seats.len() - row_length;
    let is_left_wall = seat % row_length == 0;
    let is_right_wall = seat % row_length == row_length - 1;

    if !is_top_row {
        if index_in_use(seat - row_length) {
            in_use_around += 1;
        }

        if !is_left_wall && index_in_use(seat - row_length - 1) {
            in_use_around += 1;
        }

        if !is_right_wall && index_in_use(seat - row_length + 1) {
            in_use_around += 1;
        }
    }

    if !is_bottom_row {
        if index_in_use(seat + row_length) {
            in_use_around += 1;
        }

        if !is_left_wall && index_in_use(seat + row_length - 1) {
            in_use_around += 1;
        }

        if !is_right_wall && index_in_use(seat + row_length + 1) {
            in_use_around += 1;
        }
    }

    if !is_left_wall && index_in_use(seat - 1) {
        in_use_around += 1;
    }

    if !is_right_wall && index_in_use(seat + 1) {
        in_use_around += 1;
    }

    in_use_around
}

fn tick(all_seats: &Vec<LocationState>, row_length: usize) -> Vec<LocationState> {
    all_seats
        .iter()
        .enumerate()
        .map(|(i, current_state)| {
            match current_state {
                LocationState::EmptySeat => {
                    if count_seats_in_use_around(all_seats, row_length, i) == 0 {
                        LocationState::FilledSeat
                    } else {
                        LocationState::EmptySeat
                    }
                },
                LocationState::FilledSeat => {
                    if count_seats_in_use_around(all_seats, row_length, i) >= 4 {
                        LocationState::EmptySeat
                    } else {
                        LocationState::FilledSeat
                    }
                },
                _ => current_state.clone(),
            }
        })
        .collect()
}

fn main() -> std::io::Result<()> {
    let arg: Option<String> = args().skip(1).next();
    if arg.is_none() {
        println!("Missing arg");
        return Ok(());
    }
    let file_path = arg.unwrap();
    let mut file = File::open(file_path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    let mut all_seats: Vec<LocationState> = buffer
        .chars()
        .filter(|c| c != &'\n')
        .filter_map(|c| match c {
            'L' => Some(LocationState::EmptySeat),
            '.' => Some(LocationState::Floor),
            _ => None
        })
        .collect();
    let row_length = buffer.split("\n").nth(0).unwrap().len();

    let mut prev_seats_in_use = 1;
    let mut cur_seats_in_use = 0;
    while prev_seats_in_use != cur_seats_in_use {
        prev_seats_in_use = cur_seats_in_use;

        all_seats = tick(&all_seats, row_length);

        println!("Tick!");
        print_state(&all_seats, row_length);

        cur_seats_in_use = count_seats_in_use(&all_seats);
    }

    println!("Final stable state with {} seats filled", cur_seats_in_use);
    print_state(&all_seats, row_length);
    
    Ok(())
}
