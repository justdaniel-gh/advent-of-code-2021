use std::fs;

pub fn read_input(input_filename: &str) -> String {
    let input_data =
        String::from_utf8(fs::read(input_filename).expect("Unable to open input!")).unwrap();

    input_data
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
