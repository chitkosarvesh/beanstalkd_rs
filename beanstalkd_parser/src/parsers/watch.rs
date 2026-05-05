use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::space1,
};

use crate::command::Command;

pub fn parse_watch(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("watch")(input)?;
    let (input, _) = space1(input)?;
    let (input, tube) = take_until("\r\n")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::Watch { tube }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_watch() {
        let input = "watch my-tube\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Watch { tube: "my-tube" });
    }
}
