use std::fs;

fn count_increases(nums: &Vec<i32>) -> i32 {
    let mut num_increases: i32 = 0;
    let mut last = nums[0];
    for num in nums.iter().skip(1) {
        if num > &last {
            num_increases += 1;
        }
        last = *num;
    }
    num_increases
}

fn read_input() -> Vec<i32> {
    let input_filename = "data/input.txt";
    let input_data =
        String::from_utf8(fs::read(input_filename).expect("Unable to open input!")).unwrap();
    println!("Got text! Size: {}\n", input_data.len());

    let input_nums: Vec<i32> = input_data
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    println!("Got lines! Size: {}", input_nums.len());
    input_nums
}

fn part1(input_nums: &Vec<i32>) {
    let num_increases = count_increases(input_nums);
    println!("Part1 - Number of increases: {}", num_increases);
}

fn part2(input_nums: &Vec<i32>) {
    let mut sums: Vec<i32> = Vec::new();
    let mut ndx = 0;
    for num in input_nums.iter().skip(2) {
        sums.push(*num);
        if ndx > 0 {
            sums[ndx-1] += num;
        }
        if ndx > 1 {
            sums[ndx-2] += num;
        }
        ndx += 1;
    }
    let num_increases = count_increases(&sums);
    println!("Part2 - Number of increases: {}", num_increases);
}

fn main() {
    let input_nums = read_input();
    part1(&input_nums);
    part2(&input_nums);
}