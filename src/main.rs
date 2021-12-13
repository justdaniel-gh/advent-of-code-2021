use argparse::{ArgumentParser, StoreTrue};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let mut next_day = false;
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Manage my days.");
        ap.refer(&mut next_day)
            .add_option(&["-n", "--next_day"], StoreTrue, "Start the next day.");
        //ap.refer(&mut name).add_option(&["--name"], Store, "Name for the greeting");
        ap.parse_args_or_exit();
    }

    if next_day {
        // For now, just create the project and the input file
        let path: String = "day".to_owned();
        for day in 1..26 {
            let day_name = format!("{}{}", path, &day.to_string());
            if Path::new(&day_name).is_dir() {
                continue;
            }
            let _output = Command::new("cargo")
                .args(&["init", &day_name])
                .status()
                .expect("Failed to create day!");

            fs::copy("template.rs", format!("{}/src/main.rs", day_name)).unwrap();
            fs::write(format!("data/input-{}.txt", day_name), "")
                .expect("Unable to create input file!");
            break;
        }
    }
}
