use nom::{
    IResult,
    bytes::streaming::tag,
    character::complete::{space1, u64},
};

use crate::command::Command;

pub fn parse_peek(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("peek")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::Peek { id }))
}
pub fn parse_peek_ready(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("peek-ready")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::PeekReady))
}
pub fn parse_peek_delayed(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("peek-delayed")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::PeekDelayed))
}
pub fn parse_peek_buried(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("peek-buried")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::PeekBuried))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_peek() {
        let input = "peek 42\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Peek { id: 42 });
    }
    #[test]
    fn test_peek_ready() {
        let input = "peek-ready\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::PeekReady);
    }
    #[test]
    fn test_peek_delayed() {
        let input = "peek-delayed\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::PeekDelayed);
    }
    #[test]
    fn test_peek_buried() {
        let input = "peek-buried\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::PeekBuried);
    }
}
