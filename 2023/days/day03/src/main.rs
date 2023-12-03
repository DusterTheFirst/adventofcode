use std::ops::Sub;

fn main() {
    let file = std::fs::read_to_string("input/input").expect("input file should exist");

    let mut numbers = Vec::<Span>::new();
    let mut symbols = Vec::<(char, Point)>::new();
    for (y, line) in file.lines().enumerate() {
        let mut current_span = None;

        for (x, char) in line.char_indices() {
            if let Some(digit) = char.to_digit(10) {
                let current_span = current_span.get_or_insert(Span {
                    length: 0,
                    start: Point { x, y },
                    value: 0,
                });

                current_span.value = current_span.value * 10 + digit;
                current_span.length += 1;
            } else {
                if let Some(span) = current_span.take() {
                    numbers.push(span);
                }

                if char != '.' {
                    symbols.push((char, Point { x, y }));
                }
            }
        }

        if let Some(span) = current_span.take() {
            numbers.push(span);
        }
    }

    // dbg!(&numbers, &symbols);

    let sum: u32 = symbols
        .iter()
        .flat_map(|(symbol, pos)| {
            numbers
                .iter()
                .copied()
                .filter(move |span| span.is_adjacent(*pos))
                .map(move |span| (symbol, span))
        })
        // .inspect(|(symbol, span)| println!("{symbol} {span:?}"))
        .map(|(_, span)| span.value)
        .sum();

    dbg!(sum);

    let gears: u32 = symbols
        .into_iter()
        .filter_map(|(symbol, pos)| {
            if symbol != '*' {
                return None;
            }

            let numbers = numbers
                .iter()
                .filter(move |span| span.is_adjacent(pos))
                .collect::<Vec<_>>();

            if numbers.len() == 2 {
                Some(numbers[0].value * numbers[1].value)
            } else {
                None
            }
        })
        .sum();

    dbg!(gears);
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Sub for Point {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x as isize - rhs.x as isize,
            y: self.y as isize - rhs.y as isize,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    value: u32,
    start: Point,
    length: usize,
}

impl Span {
    pub fn end(&self) -> Point {
        Point {
            x: self.start.x + self.length,
            y: self.start.y,
        }
    }

    pub fn is_adjacent(&self, point: Point) -> bool {
        let diff = point - self.start;

        let vertical_range = -1..=1;
        let horizontal_range = -1..=(self.length as isize);

        vertical_range.contains(&diff.y) && horizontal_range.contains(&diff.x)
    }
}
