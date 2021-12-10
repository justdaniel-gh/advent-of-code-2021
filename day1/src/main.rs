use common;

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

fn part1(input_nums: &Vec<i32>) {
    let num_increases = count_increases(input_nums);
    println!("Part1 - Number of increases: {}", num_increases);
}

fn part2(input_nums: &Vec<i32>) {
    let mut sums: Vec<i32> = Vec::new();
    let mut ndx = 0;
    for num in input_nums {
        sums.push(*num);
        if ndx > 0 {
            sums[ndx - 1] += num;
        }
        if ndx > 1 {
            sums[ndx - 2] += num;
        }
        ndx += 1;
    }
    let num_increases = count_increases(&sums);
    println!("Part2 - Number of increases: {}", num_increases);
}

fn main() {
    let input_data = common::read_input("data/input-day1.txt");
    let input_nums: Vec<i32> = input_data
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    part1(&input_nums);
    part2(&input_nums);
}
