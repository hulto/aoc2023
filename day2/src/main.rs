use std::{io::{BufReader, BufRead}, fs::File, collections::HashMap};

use anyhow::{Result, Context};


#[derive(Debug,PartialEq,Clone)]
struct Bag {
    data: HashMap<Color, u32>   
}

impl Default for Bag {
    fn default() -> Self {
        Bag {
            data: HashMap::from([
                (Color::RED, 0),
                (Color::BLUE, 0),
                (Color::GREEN, 0)
            ])
        }
    }
}

#[derive(Debug,PartialEq)]
struct Game {
    index: u32,
    data: Vec<GameSet>
}
#[derive(Debug,PartialEq)]
struct GameSet {
    data: HashMap<Color, u32>
}
#[derive(Debug,Eq, Hash, PartialEq, Clone)]
enum Color {
    RED,
    BLUE,
    GREEN,
}

fn validate_game(bag: Bag, game: &Game) -> Result<bool> {
    println!("game: {:?}\nbag: {:?}", game, bag);
    let queried_bag = calculate_minimum_bag(game)?;
    for (color, found_count) in queried_bag.data {
        let max_count = bag.data.get(&color).context("Color not found")?;
        if found_count > *max_count {
            return Ok(false);
        }
    }
    Ok(true)
}

fn calculate_minimum_bag(game: &Game) -> Result<Bag> {
    let mut queried_bag = Bag::default();
    for set in game.data.iter() {
        for (color, count) in set.data.iter() {
            queried_bag
                .data
                .entry(color.clone())
                .and_modify(|cur_count| if *count > *cur_count { *cur_count = *count } );
        }
    }
    Ok(queried_bag)
}

fn calculate_power(bag: Bag) -> Result<u32> {
    let red = *bag.data.get(&Color::RED).context("Unable to get RED")?;
    let green = *bag.data.get(&Color::GREEN).context("Unable to get GREEN")?;
    let blue = *bag.data.get(&Color::BLUE).context("Unable to get BLUE")?;
    Ok(red * green * blue)
}



fn parse_game_data(game_row: String) -> Result<Game> {
    let mut colon_split = game_row.trim().split(":");
    let index_str_data = colon_split.nth(0).context("index_str_data unable to get 0th element")?;
    let index_str = index_str_data.split(" ").nth(1).context("index_str unable to get 1st element")?;
    let index = index_str.parse::<u32>()?;

    let mut data = Vec::new();
    let set_str = colon_split.nth(0).context("set_str unable to get 0th element")?;
    for set in set_str.trim().split(";") {
        let mut set_hash_map: HashMap<Color, u32> = HashMap::new();
        for cube in set.split(",") {
            let cube_split: Vec<&str> = cube.trim().split(" ").collect();
            let count = cube_split[0].parse::<u32>()?;
            let color = match cube_split[1] {
                "blue" => Color::BLUE,
                "red" => Color::RED,
                "green" => Color::GREEN,
                _ => return Err(anyhow::anyhow!("Unknown color"))
            };
            set_hash_map.entry(color)
                .and_modify(|current_count| *current_count+=count)
                .or_insert(count);

        }
        data.push(GameSet {
            data: set_hash_map
        });
    }

    Ok(Game{
        index,
        data
    })
}

fn parse_all_game_data(reader: BufReader<File>) -> Result<Vec<Game>> {
    let mut res: Vec<Game> = Vec::new();
    for line in reader.lines() {
        res.push(parse_game_data(line?)?);
    }
    Ok(res)
}

fn part1(data_path: String) -> Result<u32>{
    let file = File::open(data_path)?;
    let reader = BufReader::new(file);
    let games = parse_all_game_data(reader)?;
    //12 red cubes, 13 green cubes, and 14 blue cubes
    let bag = Bag {
        data: HashMap::from([
            (Color::RED, 12),
            (Color::GREEN, 13),
            (Color::BLUE, 14),
        ])
    };
    let mut res = 0;
    for game in games {
        if validate_game(bag.clone(), &game)? {
            res += game.index;
        }
    }
    Ok(res)
}

fn part2(data_path: String) -> Result<u32>{
    let file = File::open(data_path)?;
    let reader = BufReader::new(file);
    let games = parse_all_game_data(reader)?;

    let mut res = 0;
    for game in games {
        let min_bag = calculate_minimum_bag(&game)?;
        res += calculate_power(min_bag)?;
    }
    Ok(res)
}

fn main() {
    let _ = part1("input.txt".to_string()).unwrap();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;


    #[test]
    fn test_part2_solve() -> Result<()> {
        println!("{:?}", part2("input2.txt".to_string())?);
        Ok(())
    }

    #[test]
    fn test_part2_calculate_power() -> Result<()> {
        let test_bag = Bag {
            data: HashMap::from([
                (Color::BLUE, 6),
                (Color::GREEN, 2),
                (Color::RED, 4),
            ])
        };
        assert_eq!(48, calculate_power(test_bag)?);
        Ok(())
    }

    #[test]
    fn test_part1_parse_game_data() -> Result<()> {
        let test_data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 30: 13 green, 11 red, 11 blue; 7 green, 9 blue, 7 red; 11 red, 1 blue, 11 green"#;

        for (index, data) in test_data.split("\n").enumerate() {
            let res = parse_game_data(data.to_string())?;
            match index {
                0 => assert_eq!(res, Game{index:1, data: vec![
                    GameSet{ data: HashMap::from([(Color::BLUE, 3), (Color::RED, 4)]) }, 
                    GameSet{ data: HashMap::from([(Color::RED, 1), (Color::GREEN, 2), (Color::BLUE, 6)]) }, 
                    GameSet{ data: HashMap::from([(Color::GREEN, 2)]) }, 
                    ]}
                ),
                1 => assert_eq!(res, Game{index:2, data: vec![
                    GameSet{ data: HashMap::from([(Color::BLUE, 1), (Color::GREEN, 2)]) }, 
                    GameSet{ data: HashMap::from([(Color::GREEN, 3), (Color::BLUE, 4), (Color::RED, 1)]) }, 
                    GameSet{ data: HashMap::from([(Color::GREEN, 1), (Color::BLUE, 1)]) }, 
                    ]}),
                2 => assert_eq!(res, Game{index:30, data: vec![
                    GameSet{ data: HashMap::from([(Color::GREEN, 13), (Color::RED, 11), (Color::BLUE, 11)]) }, 
                    GameSet{ data: HashMap::from([(Color::GREEN, 7), (Color::RED, 7), (Color::BLUE, 9)]) }, 
                    GameSet{ data: HashMap::from([(Color::GREEN, 11), (Color::RED, 11), (Color::BLUE, 1)]) }, 
                ]}),
                _ => assert!(false),
            }
        }
        Ok(())
    }
    #[test]
    fn test_part1_check_game() -> Result<()> {
        //Game 30: 13 green, 11 red, 11 blue; 7 green, 9 blue, 7 red; 11 red, 1 blue, 11 green

        let test_data = vec![
            Game{index:1, data: vec![
                GameSet{ data: HashMap::from([(Color::BLUE, 3), (Color::RED, 4)]) }, 
                GameSet{ data: HashMap::from([(Color::RED, 1), (Color::GREEN, 2), (Color::BLUE, 6)]) }, 
                GameSet{ data: HashMap::from([(Color::GREEN, 2)]) }, 
            ]},
            Game{index:2, data: vec![
                GameSet{ data: HashMap::from([(Color::BLUE, 1), (Color::GREEN, 2)]) }, 
                GameSet{ data: HashMap::from([(Color::GREEN, 3), (Color::BLUE, 20), (Color::RED, 1)]) }, 
                GameSet{ data: HashMap::from([(Color::GREEN, 1), (Color::BLUE, 1)]) }, 
            ]},
            Game{index:30, data: vec![
                GameSet{ data: HashMap::from([(Color::GREEN, 13), (Color::RED, 11), (Color::BLUE, 11)]) }, 
                GameSet{ data: HashMap::from([(Color::GREEN, 7), (Color::RED, 7), (Color::BLUE, 9)]) }, 
                GameSet{ data: HashMap::from([(Color::GREEN, 11), (Color::RED, 11), (Color::BLUE, 1)]) }, 
            ]}
        ];


        for (index, game) in test_data.iter().enumerate() {
            let bag = Bag {
                data: HashMap::from([
                    (Color::RED, 12),
                    (Color::BLUE, 14),
                    (Color::GREEN, 13)
                ])
            };
    
            match index {
                0 => assert_eq!(validate_game(bag, game)?, true),
                1 => assert_eq!(validate_game(bag, game)?, false),
                2 => assert_eq!(validate_game(bag, game)?, true),
                _ => assert!(false),    
            }
        }

        Ok(())
    }

    #[test]
    fn test_part1_solve() -> Result<()> {
        assert_eq!(2207, part1("input.txt".to_string())?);
        Ok(())
    }
}
