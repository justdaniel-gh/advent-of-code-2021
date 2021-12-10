use common;
use std::cmp::Ordering;

fn bitwise_not(value: usize, bits: usize) -> usize {
    let noted = (1 << (bits)) - 1;
    value ^ noted
}

fn get_bit_at(value: usize, bit_num: usize) -> usize {
    (value & (1 << bit_num - 1)) >> (bit_num - 1)
}

// Go through bit_slice and filter down to the value that matches the max_bit_num pattern
fn there_can_be_only_one(
    bit_slice: &[usize],
    num_bits: usize,
    equal_value: usize,
    use_least: bool,
) -> &[usize] {
    if bit_slice.len() == 1 || num_bits == 0 {
        return bit_slice;
    }
    let max_bit_num = calc_bit_match_pattern(&bit_slice, num_bits, equal_value, use_least);
    let bit_value = get_bit_at(max_bit_num, num_bits);
    // Since the numbers are sorted, find the start bit that matches, collect until the bit changes
    let start_ndx: usize = bit_slice
        .iter()
        .position(|&v| get_bit_at(v, num_bits) == bit_value)
        .expect("Oopsie...Did you sort it?");
    let end_ndx: usize = bit_slice[start_ndx..]
        .iter()
        .position(|v| get_bit_at(*v, num_bits) != bit_value)
        .unwrap_or(bit_slice.len());
    return there_can_be_only_one(
        &bit_slice[start_ndx..end_ndx],
        num_bits - 1,
        equal_value,
        use_least,
    );
}

/*
 This produces a number, where a 1/0 in the bit position means 1/0 was the desired value for that position
*/
fn calc_bit_match_pattern(
    input_nums: &[usize],
    max_bit_count: usize,
    equal_value: usize,
    use_least: bool,
) -> usize {
    let mut bit_count = vec![0; max_bit_count];
    let mut max_bit_num: usize = 0;
    let num_count: usize = input_nums.len();
    // Calculate count of bits in each position
    for num in input_nums {
        for ndx in 0..max_bit_count {
            bit_count[ndx] += (num & (1 << (max_bit_count - 1 - ndx))) >> (max_bit_count - 1 - ndx);
        }
    }
    for count in bit_count {
        match (count as f32)
            .partial_cmp(&(num_count as f32 / 2.0))
            .expect("I can't compare these for some reason!")
        {
            Ordering::Greater => max_bit_num += if use_least { 0 } else { 1 },
            Ordering::Less => max_bit_num += if use_least { 1 } else { 0 },
            // If they are equal, sometimes you may want 0 to be the "most common" sometimes 1
            Ordering::Equal => max_bit_num += equal_value,
        }
        max_bit_num = max_bit_num << 1;
    }
    // There was an extra shift...
    max_bit_num >> 1
}

fn part1(input_nums: &[usize], max_bit_count: usize) {
    let gamma_rate = calc_bit_match_pattern(&input_nums, max_bit_count, 0, false);
    let epsilon_rate = bitwise_not(gamma_rate as usize, max_bit_count);

    println!("Gamma: {}  Epsilon: {}", gamma_rate, epsilon_rate);
    println!("Power Consumption: {}", gamma_rate * epsilon_rate);
}

fn part2(mut input_nums: Vec<usize>, max_bit_count: usize) {
    input_nums.sort();
    // Find Oxygen Generator Value
    //  Most common value
    //  Equal should return 1
    let oxygen_generator_value = there_can_be_only_one(&input_nums, max_bit_count, 1, false)[0];
    // Find CO2 Scrubber value
    //  Least common value
    //  Equal should return 0
    let co2_scrubber_value = there_can_be_only_one(&input_nums, max_bit_count, 0, true)[0];
    println!(
        "Oxygen: {}  CO2 Scrubber: {}",
        oxygen_generator_value, co2_scrubber_value
    );
    println!(
        "Life Support Rating: {}",
        oxygen_generator_value * co2_scrubber_value
    );
}

fn main() {
    let data = common::read_input("data/input-day3.txt");
    let input_nums: Vec<usize> = data
        .lines()
        .map(|s| usize::from_str_radix(s, 2).unwrap())
        .collect();
    let max_bit_count = data.split_once("\n").unwrap().0.len();
    part1(&input_nums, max_bit_count);
    part2(input_nums, max_bit_count);
}
