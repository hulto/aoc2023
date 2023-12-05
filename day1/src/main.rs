use std::{fs::File, io::{BufReader, BufRead}, ops::Index};
use anyhow::{Result, Ok, Context};

const NUM_TOKENS: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "1",
    "2",
    "3",
    "4",
    "5",
    "6",
    "7",
    "8",
    "9",
    "0"
];

fn tokenize_string(line: &str) -> Result<Vec<u32>> {
    let mut res: Vec<u32> = Vec::new();
    for mut index in 0..line.len() {
        for token in NUM_TOKENS {
            if token.len() <= line.len()-index {
                if *token == line[index..index+token.len()] {
                    res.push(str_to_u32(token)?);
                    index+=token.len();
                    break;
                }
            }
        }
    }

    Ok(res)
}

fn str_to_u32(num: &str) -> Result<u32> {
    let mut res = 0;
    for (index, token) in NUM_TOKENS.iter().enumerate() {
        if num == *token {
            res = index as u32;
        }
    }
    let tmp = num.chars().nth(0).context("couldn't get zeroth")?;
    if tmp.is_numeric() {
        res = tmp.to_digit(10).context("Failed to covert")?;
    }
    Ok(res)
}

fn decode_value(line: &str) -> Result<u32> {
    let mut line_vec: Vec<char> = line.chars().collect();
    line_vec.retain(|a| a.is_numeric());

    let tens_digit = line_vec[0].to_digit(10).context("Failed to convert")?;
    let ones_digit = line_vec[line_vec.len()-1].to_digit(10).context("Failed to convert")?;
    let res: u32 = tens_digit*10 + ones_digit;
    Ok(res)
}

fn part1() -> Result<u32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        sum += decode_value(line?.as_str())?;
    }

    Ok(sum)
}

fn part2() -> Result<u32> {
    let file = File::open("input2.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let tokens = tokenize_string(line?.as_str())?;
        let tens_digit = tokens[0];
        let one_digit = tokens[tokens.len()-1];
        let res = tens_digit*10+one_digit;
        sum += res;
    }

    Ok(sum)
}

fn main() -> Result<()> {
    println!("{}", part2()?);
    Ok(())
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_tokenize() -> Result<()> {
        Ok(())
    }
}