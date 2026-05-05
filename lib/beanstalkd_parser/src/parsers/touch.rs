use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space1, u64},
};

use crate::command::Command;

pub fn parse_touch(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("touch")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::Touch { id }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_touch() {
        let input = "touch 42\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Touch { id: 42 });
    }
}
