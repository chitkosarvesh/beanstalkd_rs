use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u16, u64},
};

use crate::command::Command;

pub fn parse_bury(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("bury")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, pri) = u16(input)?;
    let (input, _) = alt((tag("\r\n"), tag("\n"))).parse(input)?;
    Ok((input, Command::Bury { id, pri }))
}

#[cfg(test)]
mod tests {
    use crate::parse_command;

    use super::*;
    #[test]
    fn test_bury() {
        let input = "bury 42 1\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Bury { id: 42, pri: 1 });
    }
}
