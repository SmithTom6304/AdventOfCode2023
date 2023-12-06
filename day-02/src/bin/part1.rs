use day_02::data::{CubeGroup, Game};

fn main() {
    println!("Hello, world!");
}

pub fn is_game_possible(game: Game, bag: CubeGroup) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{CubeGroup, Game};

    #[test]
    fn can_deserialize_data() {
        let expected_id = 1;
        let expected_groups = [
            CubeGroup {
                red_cubes: 4,
                green_cubes: 0,
                blue_cubes: 3,
            },
            CubeGroup {
                red_cubes: 1,
                green_cubes: 2,
                blue_cubes: 6,
            },
            CubeGroup {
                red_cubes: 0,
                green_cubes: 2,
                blue_cubes: 0,
            },
        ];

        let expected_game = Game {
            id: expected_id,
            reveals: expected_groups.to_vec(),
        };

        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

        assert_eq!(expected_game, Game::from_line(data));
    }
}
