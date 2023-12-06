use std::{env, fs};

use day_02::data::{CubeGroup, Game};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut sum: u32 = 0;
    let bag = CubeGroup {
        red_cubes: 12,
        green_cubes: 13,
        blue_cubes: 14,
    };
    contents.lines().into_iter().for_each(|input| {
        let game = Game::from_line(input);
        if is_game_possible(&game, &bag) {
            sum += game.id as u32;
        }
    });
    println!("Result = {}", sum);
}

pub fn is_game_possible(game: &Game, bag: &CubeGroup) -> bool {
    let iter = game.reveals.iter();
    return bag.red_cubes > iter.clone().map(|reveal| reveal.red_cubes).max().unwrap()
        && bag.green_cubes > iter.clone().map(|reveal| reveal.green_cubes).max().unwrap()
        && bag.blue_cubes > iter.clone().map(|reveal| reveal.blue_cubes).max().unwrap();
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{is_game_possible, CubeGroup, Game};

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game {id: 1, reveals: vec![
        CubeGroup { red_cubes: 4, green_cubes: 0, blue_cubes: 3 },
        CubeGroup { red_cubes: 1, green_cubes: 2, blue_cubes: 6 },
        CubeGroup { red_cubes: 0, green_cubes: 2, blue_cubes: 0 },
    ]})]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Game {id: 2, reveals: vec![
        CubeGroup { red_cubes: 0, green_cubes: 2, blue_cubes: 1 },
        CubeGroup { red_cubes: 1, green_cubes: 3, blue_cubes: 4 },
        CubeGroup { red_cubes: 0, green_cubes: 1, blue_cubes: 1 },
    ]})]
    #[case(
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        Game {id: 3, reveals: vec![
        CubeGroup { red_cubes: 20, green_cubes: 8, blue_cubes: 6 },
        CubeGroup { red_cubes: 4, green_cubes: 13, blue_cubes: 5 },
        CubeGroup { red_cubes: 1, green_cubes: 5, blue_cubes: 0 },
    ]}
    )]
    #[case(
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        Game {id: 4, reveals: vec![
        CubeGroup { red_cubes: 3, green_cubes: 1, blue_cubes: 6 },
        CubeGroup { red_cubes: 6, green_cubes: 3, blue_cubes: 0 },
        CubeGroup { red_cubes: 14, green_cubes: 3, blue_cubes: 15 },
    ]}
    )]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Game {id: 5, reveals: vec![
        CubeGroup { red_cubes: 6, green_cubes: 3, blue_cubes: 1 },
        CubeGroup { red_cubes: 1, green_cubes: 2, blue_cubes: 2 },
    ]})]
    fn can_deserialize_data(#[case] input: &str, #[case] expected: Game) {
        assert_eq!(expected, Game::from_line(input));
    }

    #[rstest]
    #[case(Game {id: 1, reveals: vec![
        CubeGroup { red_cubes: 4, green_cubes: 0, blue_cubes: 3 },
        CubeGroup { red_cubes: 1, green_cubes: 2, blue_cubes: 6 },
        CubeGroup { red_cubes: 0, green_cubes: 2, blue_cubes: 0 },
    ]}, true)]
    #[case(Game {id: 2, reveals: vec![
        CubeGroup { red_cubes: 0, green_cubes: 2, blue_cubes: 1 },
        CubeGroup { red_cubes: 1, green_cubes: 3, blue_cubes: 4 },
        CubeGroup { red_cubes: 0, green_cubes: 1, blue_cubes: 1 },
    ]}, true)]
    #[case(
        Game {id: 3, reveals: vec![
        CubeGroup { red_cubes: 20, green_cubes: 8, blue_cubes: 6 },
        CubeGroup { red_cubes: 4, green_cubes: 13, blue_cubes: 5 },
        CubeGroup { red_cubes: 1, green_cubes: 5, blue_cubes: 0 },
    ]}, false
    )]
    #[case(
        Game {id: 4, reveals: vec![
        CubeGroup { red_cubes: 3, green_cubes: 1, blue_cubes: 6 },
        CubeGroup { red_cubes: 6, green_cubes: 3, blue_cubes: 0 },
        CubeGroup { red_cubes: 14, green_cubes: 3, blue_cubes: 15 },
    ]}, false
    )]
    #[case(Game {id: 5, reveals: vec![
        CubeGroup { red_cubes: 6, green_cubes: 3, blue_cubes: 1 },
        CubeGroup { red_cubes: 1, green_cubes: 2, blue_cubes: 2 },
    ]}, true)]
    fn can_check_if_game_is_possible(#[case] game: Game, #[case] should_be_possible: bool) {
        let bag = CubeGroup {
            red_cubes: 12,
            green_cubes: 13,
            blue_cubes: 14,
        };
        assert_eq!(should_be_possible, is_game_possible(&game, &bag))
    }
}
