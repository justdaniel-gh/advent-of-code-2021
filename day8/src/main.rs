use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

struct SignalPattern {
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl FromStr for SignalPattern {
    type Err = ();

    fn from_str(input: &str) -> Result<SignalPattern, Self::Err> {
        /* Reads in a string "abc defg cda | abc cdfg"
         */
        let sort_chars = |v: &str| String::from(v).chars().sorted().collect::<String>();
        let (inputs, outputs) = input
            .split_once(" | ")
            .map(|(a, b)| {
                (
                    a.split_whitespace().map(sort_chars).collect(),
                    b.split_whitespace().map(sort_chars).collect(),
                )
            })
            .unwrap();
        Ok(SignalPattern { inputs, outputs })
    }
}

/*
    Segments:

    0:      1:      2:      3:      4:
     aaaa    ....    aaaa    aaaa    ....
    b    c  .    c  .    c  .    c  b    c
    b    c  .    c  .    c  .    c  b    c
     ....    ....    dddd    dddd    dddd
    e    f  .    f  e    .  .    f  .    f
    e    f  .    f  e    .  .    f  .    f
     gggg    ....    gggg    gggg    ....

    5:      6:      7:      8:      9:
     aaaa    aaaa    aaaa    aaaa    aaaa
    b    .  b    .  .    c  b    c  b    c
    b    .  b    .  .    c  b    c  b    c
     dddd    dddd    ....    dddd    dddd
    .    f  e    f  .    f  e    f  .    f
    .    f  e    f  .    f  e    f  .    f
     gggg    gggg    ....    gggg    gggg

   0 -> [0, 1, 7]   5 -> [5]
   1 -> [1]         6 -> [5, 6]
   2 -> [2]         7 -> [1, 7]
   3 -> [1, 3, 7]   8 -> [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
   4 -> [1, 4]      9 -> [1, 3, 4, 5, 7, 9]

   Length   Numbers
   2        1
   3        7
   4        4
   5        2, 3, 5
   6        0, 6, 9
   7        8

    2,3,5 all have a,d,g -- 4 has d, 7 has a

    2 is going to be missing 2 values from 4
    3 is going to be missing 1 value from 4, but have all of 7s
    5 is going to be missing 1 value from 4 and 7
*/

fn process_pattern(signal_pattern: &SignalPattern) -> usize {
    /*
                missing b,e
     |7  |6     |2    |8      |1 |4   |  5  |0     |9     |3     | 3     3     8       5
     |dbc|gfecab|afcdg|dfebcag|bd|dgbe|bcaeg|dcefab|ecgadb|agcbd | acdgb gbcda gdecfba bacge
     |cfa|debagf|geacd|cebfagd|fc|cdfb|fagbd|cabegf|badgcf|gdafc | gacdf
     |   |      |     |       |  |    |     |      |      |
     |   |      |     |       |  |    |     |      |      |
       Find a by diffing 7 & 1
       Find 2 by diffing all [5 len] contents to 4's values, the one that is 2 off is 2
       Find 3 by diffing remaining [5 len] with 7's values, where it == 0, that's 3
       Find 5 because it is the only [5 len] one left
       Find b by diffing 5 and 3; Result->Left
       Find c by diffing 5 and 3; Result->Right
       Find f by removing `c` from 1
       Find d by removing knowns from 4
       Find g by removing knowns from 3
       Find 0 because its the only [6 len] without d in it
       Find 9 because its the only [6 len] with no unknowns
       Find 6 because it is the only [6 len] one left
       Find e because its the only one left in 8

       a = value in 7 that is not in 1
       b = value in 5 that is not in 3
       c = value in 3 that is not in 5
       d = value left in 4
       e = value left in 8
       f = value left in 1 or 7
       g = value left in 3
    */

    /*
      Step 1: Sort each input a->g
      Step 2: Find the uniques, assign the scrambled inputs to those numbers
      Step 3:

    */
    let mut signal_to_segment_map: HashMap<char, char> = HashMap::new();
    let return_diff_left = |a, b| {
        diff::chars(a, b)
            .into_iter()
            .find_map(|v| match v {
                diff::Result::Left(left) => Some(left),
                _ => None,
            })
            .expect("Unable to find match")
    };

    let empty = String::default();
    let mut number_to_pattern: Vec<&String> = vec![&empty; 10];
    for input in &signal_pattern.inputs {
        match input.len() {
            2 => number_to_pattern[1] = input, // #1
            4 => number_to_pattern[4] = input, // #4
            3 => number_to_pattern[7] = input, // #7
            7 => number_to_pattern[8] = input, // #8
            _ => (),
        };
    }
    // 7.diff(1) => Left() maps 'a'
    signal_to_segment_map.insert(
        'a',
        return_diff_left(number_to_pattern[7], number_to_pattern[1]),
    );
    // Find 2 by diffing all [5 len] contents to 4's values, the one that is 2 off is 2
    // Find 3 by diffing remaining [5 len] with 7's values, where it == 0, that's 3
    // Find 5 because it is the only [5 len] one left
    for input in &signal_pattern.inputs {
        if input.len() == 5 {
            match diff::chars(input, number_to_pattern[4])
                .iter()
                .fold(0, |s, v| {
                    s + match v {
                        // Add up the number of rights only (segments that are unique to 4)
                        diff::Result::Right(_) => 1,
                        _ => 0,
                    }
                }) {
                1 => match diff::chars(input, number_to_pattern[7])
                    .iter()
                    .fold(0, |s, v| {
                        s + match v {
                            diff::Result::Both(_, _) => 1,
                            _ => 0,
                        }
                    }) {
                    3 => number_to_pattern[3] = input,
                    _ => number_to_pattern[5] = input,
                },
                2 => number_to_pattern[2] = input, // Whichever one has 2 is 2!
                _ => (),
            }
        }
    }
    // Now have 1,4,7,8,a,2,3,5
    // Find b by diffing 5 and 3; Result->Left
    // Find c by diffing 5 and 3; Result->Right
    for result in diff::chars(number_to_pattern[5], number_to_pattern[3]).iter() {
        match result {
            diff::Result::Left(left) => signal_to_segment_map.insert('b', *left),
            diff::Result::Right(right) => signal_to_segment_map.insert('c', *right),
            _ => None,
        };
    }
    // Find f by removing `c` from 1
    //  This is not a cool way to do this....
    if number_to_pattern[1].chars().next().unwrap() == signal_to_segment_map[&'c'] {
        signal_to_segment_map.insert('f', number_to_pattern[1].chars().nth(1).unwrap());
    } else {
        signal_to_segment_map.insert('f', number_to_pattern[1].chars().next().unwrap());
    }
    // Now have 1,4,7,8,a,2,3,5,b,c,f
    // Find d by ignoring knowns from 4
    for chr in number_to_pattern[4].chars() {
        match signal_to_segment_map.values().any(|&c| c == chr) {
            // Known value? Not interested
            true => (),
            // Unknown! It's d
            _ => {
                signal_to_segment_map.insert('d', chr);
            }
        }
    }

    // Find 0 because its the only [6 len] without d in it
    // Find 9 because it has c in it
    // Find 6 because it is the only [6 len] one left

    for input in &signal_pattern.inputs {
        if input.len() == 6 {
            match input.find(|v| v == signal_to_segment_map[&'d']) {
                Some(_) => {
                    // Has d in it, 6 or 9
                    match input.find(|v| v == signal_to_segment_map[&'c']) {
                        // Has c in it, 9
                        Some(_) => number_to_pattern[9] = input,
                        _ => number_to_pattern[6] = input,
                    }
                }
                _ => number_to_pattern[0] = input,
            }
        }
    }

    let final_sum = signal_pattern
        .outputs
        .iter()
        .enumerate()
        .fold(0, |s, (s_n, v)| {
            s + match number_to_pattern
                .iter()
                .enumerate()
                .find(|(_, &pat)| pat == v)
            {
                Some((n, _)) => n * ((10_usize).pow(3 - s_n as u32)),
                _ => 0,
            }
        });
    final_sum
}

fn part2(signal_patterns: &[SignalPattern]) -> usize {
    let output_sum = signal_patterns
        .iter()
        .fold(0, |sum, pat| sum + process_pattern(pat));
    println!("Total output sum: {}", output_sum);
    output_sum
}

fn part1(signal_patterns: &[SignalPattern]) -> u32 {
    // 1 = 2 segments
    // 4 = 4 segments
    // 7 = 3 segments
    // 8 = 7 segments
    let num_easy = signal_patterns.iter().fold(0, |t, v| {
        t + v.outputs.iter().fold(0, |s, o| {
            s + match o.len() {
                2 => 1, // #1
                4 => 1, // #4
                3 => 1, // #7
                7 => 1, // #8
                _ => 0,
            }
        })
    });
    println!("Number of 1,4,7,8: {}", num_easy);
    num_easy
}

fn main() {
    let data = common::read_input("data/input-day8.txt");
    let signal_patterns: Vec<SignalPattern> = data
        .lines()
        .map(|line| SignalPattern::from_str(line).unwrap())
        .collect();
    println!("Part 1:");
    part1(&signal_patterns);
    println!("Part 2:");
    part2(&signal_patterns);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let data = common::read_input("data/input-day8.txt");
        let signal_patterns: Vec<SignalPattern> = data
            .lines()
            .map(|line| SignalPattern::from_str(line).unwrap())
            .collect();
        assert_eq!(part1(&signal_patterns), 476);
    }
    #[test]
    fn part2_works() {
        let data = common::read_input("data/input-day8.txt");
        let mut signal_patterns: Vec<SignalPattern> = data
            .lines()
            .map(|line| SignalPattern::from_str(line).unwrap())
            .collect();
        assert_eq!(part2(&mut signal_patterns), 1011823);
    }
}
