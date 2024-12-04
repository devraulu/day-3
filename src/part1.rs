use anyhow::Result;
use regex::Regex;
use std::{i64, io::BufRead, ops::Mul};

pub fn process(reader: impl BufRead) -> Result<i64> {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

    let sum = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            re.captures_iter(&line)
                .map(|caps| {
                    let (_, [first, second]) = caps.extract();
                    let first: i32 = first.parse().unwrap();
                    let second: i32 = second.parse().unwrap();

                    (first as i64).mul(second as i64)
                })
                .sum::<i64>()
        })
        .sum();

    Ok(sum)
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn should_parse() {
        let input = "?% mul(70,6)why() %how(700,1)mul(50,20);";
        // where())#}from()>how()mul(611,372)}{~^?>from()^mul(835,665)who()]#^don't()select()select())mul(724,851)[>&mul(188,482)$mul(781,111)[who()<why(),!]mul(678,13)why()$#%who()mul(620,771)<
        //
        let reader = Cursor::new(input);

        let result = process(reader).unwrap();

        assert_eq!(1420, result);
    }
}
