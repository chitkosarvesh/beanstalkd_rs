use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{space1, u64},
};

use crate::command::Command;

pub fn parse_delete(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("delete")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = alt((tag("\r\n"), tag("\n"))).parse(input)?;
    Ok((input, Command::Delete { id }))
}

#[cfg(test)]
mod tests {
    use crate::parse_command;

    use super::*;
    #[test]
    fn test_delete() {
        let input = "delete 42\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Delete { id: 42 });
    }
}
