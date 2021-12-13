use common;

fn part1(data: &str) {
    let lines: Vec<Line> = data
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .filter(|l| l.point_a.x == l.point_b.x || l.point_a.y == l.point_b.y)
        .collect();
    println!("Part 1:");
}

fn part2(data: &str) {
    let lines: Vec<Line> = data
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect();
    println!("Part 2:");
}

fn main() {
    let data = common::read_input("data/input-day???.txt");
    part1(&data);
    part2(&data);
}