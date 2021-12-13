use common;

fn part1(crab_starting_positions: &mut [u32]) -> u32 {
    // Need to find the cheapest cost to move all crabs to some position
    // Find the median distance each crab is away from each other
    //  Move all crabs to that position
    crab_starting_positions.sort();
    let median_dist = crab_starting_positions[crab_starting_positions.len() / 2];
    let fuel_used: u32 = crab_starting_positions.iter().fold(0, |fuel, v| {
        fuel + (*v as i32 - median_dist as i32).abs() as u32
    });
    println!("Horizontal Pos: {}  Fuel Used: {}", median_dist, fuel_used);
    fuel_used
}

fn accumulate(val: u32) -> u32 {
    if val == 0 {
        return 0;
    }
    val + accumulate(val - 1)
}

fn part2(crab_starting_positions: &mut [u32]) -> u32 {
    // Need to find the cheapest cost to move all crabs to some position
    // Now fuel cost is now summation of each move .. f(# moves) = # moves + f(# moves - 1)
    // Find the mean distance each crab is away from each other
    //  Move all crabs to that position
    //  94813677 == Too High
    let total_dist = crab_starting_positions.iter().fold(0, |d, v| d + v);
    let mean_dist = (total_dist as f32 / crab_starting_positions.len() as f32).floor() as u32;
    let fuel_used: u32 = crab_starting_positions.iter().fold(0, |fuel, v| {
        fuel + accumulate((*v as i32 - mean_dist as i32).abs() as u32)
    });
    println!("Horizontal Pos: {}  Fuel Used: {}", mean_dist, fuel_used);
    fuel_used
}

fn main() {
    let mut crab_h_poss = common::parse_list_ints_from_file("data/input-day7.txt");
    println!("Part 1:");
    part1(&mut crab_h_poss);
    println!("Part 2:");
    part2(&mut crab_h_poss);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_works() {
        let mut crab_h_poss = common::parse_list_ints_from_file("data/input-day7.txt");
        assert_eq!(part1(&mut crab_h_poss), 336040);
        assert_eq!(part2(&mut crab_h_poss), 94813675);
    }
    #[test]
    fn check_accum() {
        assert_eq!(accumulate(5 - 1), 10);
        assert_eq!(accumulate(16 - 5), 10);
    }
}
