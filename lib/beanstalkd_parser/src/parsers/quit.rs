use nom::{IResult, bytes::complete::tag};

use crate::command::Command;

pub fn parse_quit(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("quit\r\n")(input)?;
    Ok((input, Command::Quit))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_quit() {
        let input = "quit\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Quit);
    }
}
