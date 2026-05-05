use nom::{IResult, bytes::complete::tag, character::complete::space1, character::complete::u16};

use crate::command::Command;

pub fn parse_reserve(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("reserve\r\n")(input)?;
    Ok((input, Command::Reserve))
}
pub fn parse_reserve_with_timeout(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("reserve-with-timeout")(input)?;
    let (input, _) = space1(input)?;
    let (input, seconds) = u16(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::ReserveWithTimeout { seconds }))
}

#[cfg(test)]
mod tests {
    use crate::parse_command;

    use super::*;
    #[test]
    fn test_reserve() {
        let input = "reserve\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Reserve);
    }
    #[test]
    fn test_reserve_with_timeout() {
        let input = "reserve-with-timeout 5\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::ReserveWithTimeout { seconds: 5 });
    }
}
