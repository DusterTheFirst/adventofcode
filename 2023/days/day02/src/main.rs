use std::cmp::Ordering;

use nom::IResult;

fn main() {
    let file = std::fs::read_to_string("input/input").expect("input file should exist");

    part_one(&file);
    part_two(&file);
}

fn part_one(file: &str) {
    let target_view = View {
        red: 12,
        green: 13,
        blue: 14,
    };

    println!("{target_view:?}");

    let valid_games = file
        .lines()
        .map(|line| {
            let (_, game) = game(line).expect("game lines should be parsable");

            game
        })
        .filter(|game| game.views.iter().copied().all(|view| view <= target_view));

    let id_sum: u32 = valid_games.map(|game| game.id).sum();

    dbg!(id_sum);
}

fn part_two(file: &str) {
    let power = file
        .lines()
        .map(|line| {
            let (_, game) = game(line).expect("game lines should be parsable");

            let min_possible_colors = game
                .views
                .into_iter()
                .reduce(|max, view| View {
                    red: view.red.max(max.red),
                    green: view.green.max(max.green),
                    blue: view.blue.max(max.blue),
                })
                .unwrap();

            min_possible_colors.red * min_possible_colors.green * min_possible_colors.blue
        })
        .sum::<u32>();

    dbg!(power);
}

#[derive(Debug)]
struct Game {
    id: u32,
    views: Vec<View>,
}

fn number(input: &str) -> IResult<&str, u32> {
    use nom::character::complete::*;
    use nom::combinator::*;

    map_res(digit1, str::parse::<u32>)(input)
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

fn color(input: &str) -> IResult<&str, Color> {
    use nom::branch::*;
    use nom::bytes::complete::*;
    use nom::combinator::*;

    alt((
        map(tag("red"), |_| Color::Red),
        map(tag("green"), |_| Color::Green),
        map(tag("blue"), |_| Color::Blue),
    ))(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct View {
    red: u32,
    green: u32,
    blue: u32,
}

impl Ord for View {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let red = self.red.cmp(&other.red);
        let green = self.green.cmp(&other.green);
        let blue = self.blue.cmp(&other.blue);

        match (red, green, blue) {
            (Ordering::Greater, _, _) | (_, Ordering::Greater, _) | (_, _, Ordering::Greater) => {
                Ordering::Greater
            }
            (Ordering::Equal, Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Less, _, _) | (_, Ordering::Less, _) | (_, _, Ordering::Less) => {
                Ordering::Less
            }
        }
    }
}

impl PartialOrd for View {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn view(input: &str) -> IResult<&str, View> {
    use nom::bytes::complete::*;
    use nom::character::complete::*;
    use nom::multi::*;
    use nom::sequence::*;

    let (rest, colors) =
        separated_list1(tag(", "), separated_pair(number, multispace1, color))(input)?;

    let (mut red, mut green, mut blue) = (0, 0, 0);
    for (count, color) in colors {
        match color {
            Color::Red => red = count,
            Color::Green => green = count,
            Color::Blue => blue = count,
        }
    }

    Ok((rest, View { red, green, blue }))
}

fn game(input: &str) -> IResult<&str, Game> {
    use nom::bytes::complete::*;
    use nom::combinator::*;
    use nom::multi::*;
    use nom::sequence::*;

    let mut parser = all_consuming(tuple((
        tag("Game "),
        number,
        tag(": "),
        separated_list1(tag("; "), view),
    )));

    let (input, (_, game_id, _, views)) = parser(input)?;

    Ok((input, Game { id: game_id, views }))
}
