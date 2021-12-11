use std::fs;

pub fn read_input(input_filename: &str) -> String {
    let input_data =
        String::from_utf8(fs::read(input_filename).expect("Unable to open input!")).unwrap();

    input_data
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
