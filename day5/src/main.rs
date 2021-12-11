use common;
use std::iter::Iterator;
use std::str::FromStr;
use vector2d::Vector2D;

struct Line {
    point_a: Vector2D<i32>,
    point_b: Vector2D<i32>,
}

impl FromStr for Line {
    type Err = ();

    fn from_str(input: &str) -> Result<Line, Self::Err> {
        /* Reads in a string "x1,y1 -> x2,y2"
         */
        let (point_a, point_b) = input
            .split_once(" -> ")
            .map(|(a, b)| {
                let (x1, y1) = a
                    .split_once(",")
                    .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                    .unwrap();
                let (x2, y2) = b
                    .split_once(",")
                    .map(|(x, y)| (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap()))
                    .unwrap();
                (Vector2D { x: x1, y: y1 }, Vector2D { x: x2, y: y2 })
            })
            .unwrap();
        Ok(Line {
            point_a: point_a,
            point_b: point_b,
        })
    }
}

struct LineIter<'a> {
    start_vec: &'a Vector2D<i32>,
    end_vec: &'a Vector2D<i32>,
    last_vec: Option<Vector2D<i32>>,
    slope: Vector2D<i32>,
}

impl Line {
    fn iter(&self) -> LineIter<'_> {
        let mut slope = Vector2D {
            x: (self.point_b.x - self.point_a.x) as f32,
            y: (self.point_b.y - self.point_a.y) as f32,
        };
        slope = slope.normalise();
        // Convert slope to whole ints
        let slope = Vector2D {
            x: slope.x.round() as i32,
            y: slope.y.round() as i32,
        };
        LineIter {
            start_vec: &self.point_a,
            end_vec: &self.point_b,
            last_vec: None,
            slope: slope.into_vec2d(),
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Vector2D<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.last_vec {
            None => {
                self.last_vec = Some(*self.start_vec);
            }
            Some(last) => {
                let vec: Vector2D<i32> = last - *self.end_vec;
                if vec.length_squared() == 0 {
                    return None;
                }
                self.last_vec = Some(last + self.slope);
            }
        }
        self.last_vec
    }
}

struct Grid {
    cells: Vec<u16>,
    num_rows: i32,
    num_cols: i32,
}

impl Grid {
    fn mark_line(&mut self, line: &Line) -> bool {
        for point in line.iter() {
            self.cells[((point.x * self.num_cols) + point.y) as usize] += 1;
            if point.x > self.num_cols || point.y > self.num_rows {
                panic!("You broke it!");
            }
        }
        true
    }
}

fn map_lines(lines: &[Line]) {
    let max_vec = lines.iter().fold(Vector2D { x: 0, y: 0 }, |mut m, l| {
        if l.point_a.x > m.x {
            m.x = l.point_a.x;
        };
        if l.point_a.y > m.y {
            m.y = l.point_a.y;
        };
        if l.point_b.x > m.x {
            m.x = l.point_b.x;
        };
        if l.point_b.y > m.y {
            m.y = l.point_b.y;
        };
        m
    });
    let mut grid = Grid {
        cells: vec![0; (max_vec.x as usize * max_vec.y as usize)+max_vec.y as usize],
        num_rows: max_vec.y,
        num_cols: max_vec.x,
    };
    for line in lines {
        grid.mark_line(line);
    }
    let mut more_than_two_count = 0;
    for cell in grid.cells {
        if cell >= 2 {
            more_than_two_count += 1;
        }
    }
    println!(
        "Number of cells with overlapping lines: {}",
        more_than_two_count
    );
}

fn part1(data: &str) {
    let lines: Vec<Line> = data
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .filter(|l| l.point_a.x == l.point_b.x || l.point_a.y == l.point_b.y)
        .collect();
    println!("Part 1:");
    map_lines(&lines);
}

fn part2(data: &str) {
    let lines: Vec<Line> = data
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect();
    println!("Part 2:");
    map_lines(&lines);
}

fn main() {
    let data = common::read_input("data/input-day5.txt");
    part1(&data);
    part2(&data);
}
