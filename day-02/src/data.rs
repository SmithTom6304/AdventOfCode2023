use std::fmt;

use serde::de::{self, IntoDeserializer, Visitor};
use serde::Deserialize;
use serde::{self, Deserializer};

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u8,
    pub reveals: Vec<CubeGroup>,
}

impl Game {
    pub fn from_line(line: &str) -> Self {
        let game: Result<Game, serde::de::value::Error> =
            Game::deserialize(line.clone().into_deserializer());
        match game {
            Ok(g) => g,
            Err(e) => panic!("Could not deserialize game - {}", e),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CubeGroup {
    pub red_cubes: u8,
    pub green_cubes: u8,
    pub blue_cubes: u8,
}

struct GameVisitor;

impl<'de> Visitor<'de> for GameVisitor {
    type Value = Game;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a game")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let split = v.split(':');
        let strings = split.collect::<Vec<&str>>();
        let id_string = *strings.first().unwrap();
        let id = id_string
            .split(' ')
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u8>()
            .unwrap();
        let reveal_strings = strings[1].split(';').collect::<Vec<&str>>();
        let reveals: Vec<CubeGroup> = reveal_strings
            .iter()
            .map(|&reveal| {
                let result: Result<_, serde::de::value::Error> =
                    CubeGroup::deserialize(reveal.into_deserializer());
                result.unwrap()
            })
            .collect();
        Ok(Game { id, reveals })
    }
}

impl<'de> Deserialize<'de> for Game {
    fn deserialize<D>(deserializer: D) -> Result<Game, D::Error>
    where
        D: Deserializer<'de>,
    {
        let result = deserializer.deserialize_any(GameVisitor)?;
        Ok(result)
    }
}

struct CubeGroupVisitor;

impl CubeGroupVisitor {
    fn parse_cube_value(value: &str) -> u8 {
        let blue_value = *value
            .trim()
            .split(' ')
            .collect::<Vec<&str>>()
            .first()
            .unwrap();
        blue_value.parse::<u8>().unwrap()
    }
}

impl<'de> Visitor<'de> for CubeGroupVisitor {
    type Value = CubeGroup;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a cube group")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let mut red_cubes = 0;
        let mut green_cubes = 0;
        let mut blue_cubes = 0;
        let values = v.split(',');
        for value in values {
            if value.ends_with("red") {
                red_cubes = Self::parse_cube_value(value);
            }
            if value.ends_with("green") {
                green_cubes = Self::parse_cube_value(value);
            }
            if value.ends_with("blue") {
                blue_cubes = Self::parse_cube_value(value);
            }
        }
        Ok(CubeGroup {
            red_cubes,
            green_cubes,
            blue_cubes,
        })
    }
}

impl<'de> Deserialize<'de> for CubeGroup {
    fn deserialize<D>(deserializer: D) -> Result<CubeGroup, D::Error>
    where
        D: Deserializer<'de>,
    {
        let result = deserializer.deserialize_any(CubeGroupVisitor)?;
        Ok(result)
    }
}
