use common;

fn solve(init_state: &[u8], num_days: u16) {
    // Don't math twice, save it for later! (memoization ftw)
    let mut total_map: Vec<u64> = vec![0; num_days as usize];
    // Start the total at 0, because I count them in the model for the recursion exit case
    let total_fish = init_state.iter().fold(0 as u64, |sum, v| {
        sum + model(*v as u32, num_days as u32, &mut total_map)
    });
    println!(
        "After {} days there are {} lanternfish!",
        num_days,
        total_fish
    );
}

fn model(my_days:u32, num_days_rem: u32, total_map: &mut [u64]) -> u64 {
    if num_days_rem <= my_days {
        // Not enough days to reproduce, it's just me!
        return 1;
    }
    let mut total: u64 = total_map[(num_days_rem - my_days) as usize];
    if total > 0 {
        // Wait a minute, I know the answer!
        return total;
    }
    
    // Model what happens for each of my spawn for the first time it spawns... (9 days)
    total += model(9, num_days_rem-my_days, total_map);
    // ...and for every time after that (7 days)
    total += model(7, num_days_rem-my_days, total_map);
    // memoize known value
    // I know that for every case where there are {num_days_remaining} - {my number of days}, 
    //  there will ALWAYS be {total} number of fish. Store that for other iterations to use.
    total_map[(num_days_rem - my_days) as usize] = total;
    total
}

fn main() {
    let data = common::read_input("data/input-day6.txt");
    let init_state: Vec<u8> = data
        .split(',')
        .map(|line| line.parse::<u8>().unwrap())
        .collect();
    println!("Part 1:");
    solve(&init_state, 80);
    println!("Part 2:");
    solve(&init_state, 256);
}
