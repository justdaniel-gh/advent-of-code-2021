use std::fs;

pub fn read_input(input_filename: &str) -> String {
    let input_data =
        String::from_utf8(fs::read(input_filename).expect("Unable to open input!")).unwrap();

    input_data
}

pub fn parse_list_ints_from_file(input_filename: &str) -> Vec<u32> {
    let data = read_input(input_filename);
    let int_list: Vec<u32> = data
        .split(',')
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
    int_list
}

#[allow(dead_code)]
fn print_binary_values(values: &[usize]) {
    for num in values.iter() {
        println!("{:0>12b}", num);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
