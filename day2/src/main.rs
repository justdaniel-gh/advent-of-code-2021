use common;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Action {
    Forward(i32),
    Down(i32),
    Up(i32)
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        let (action, distance) = input.split_once(' ').map(|(a,b)| (a,b.parse::<i32>().unwrap())).unwrap();
        match action {
            "forward"  => Ok(Action::Forward(distance)),
            "down"  => Ok(Action::Down(distance)),
            "up"  => Ok(Action::Up(distance)),
            _      => Err(()),
        }
    }
}

fn part1(data: &str) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in data.lines() {
        let action = Action::from_str(line).unwrap();
        match action {
            Action::Forward(distance) => {
                horizontal_pos += distance;
            },
            Action::Down(distance) => {
                depth += distance
            },
            Action::Up(distance) => {
                depth -= distance
            },
        };
    }
    println!("Position: {}  Depth: {}  Total: {}", horizontal_pos, depth, horizontal_pos*depth);
}

fn part2(data: &str) {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in data.lines() {
        let action = Action::from_str(line);//.unwrap();
        match action {
            Ok(Action::Forward(distance)) => {
                /*
                forward X does two things:
                    It increases your horizontal position by X units.
                    It increases your depth by your aim multiplied by X.
                */
                horizontal_pos += distance;
                depth += aim * distance
            },
            Ok(Action::Down(distance)) => {
                // down X increases your aim by X units.
                aim += distance
            },
            Ok(Action::Up(distance)) => {
                // up X decreases your aim by X units.
                aim -= distance
            },
            Err(_) => {
                println!("You broke something! Got invalid action string: {}", line);
            }
        };
    }
    println!("Position: {}  Depth: {}  Total: {}", horizontal_pos, depth, horizontal_pos*depth);
}

fn main() {
    let data = common::read_input("data/input-day2.txt");
    part1(&data);
    part2(&data);
}