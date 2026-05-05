use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::space1,
};

use crate::command::Command;

pub fn parse_ignore(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("ignore")(input)?;
    let (input, _) = space1(input)?;
    let (input, tube) = take_until("\r\n")(input)?;
    let (input, _) = alt((tag("\r\n"), tag("\n"))).parse(input)?;
    Ok((input, Command::Ignore { tube }))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_ignore() {
        let input = "ignore my-tube\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Ignore { tube: "my-tube" });
    }
}
