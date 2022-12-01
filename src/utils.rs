use std::fs;

pub fn read_input(day: usize) -> String {
    let filename = format!("inputs/day{:0>2}", day);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", &filename))
}

pub fn read_example(day: usize, example: usize) -> String {
    let filename = format!("inputs/day{:0>2}-example{:0>2}", day, example);
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Something went wrong reading the file {}", &filename))
}
