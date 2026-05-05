use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space1, u16, u64},
};

use crate::command::Command;

pub fn parse_kick(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("kick")(input)?;
    let (input, _) = space1(input)?;
    let (input, bound) = u16(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::Kick { bound }))
}

pub fn parse_kick_job(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("kick-job")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::KickJob { id }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_kick() {
        let input = "kick 5\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Kick { bound: 5 });
    }
    #[test]
    fn test_kick_job() {
        let input = "kick-job 42\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::KickJob { id: 42 });
    }
}
