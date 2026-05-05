use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{space1, u16, u64},
};

use crate::command::Command;

pub fn parse_release(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("release")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = space1(input)?;
    let (input, pri) = u16(input)?;
    let (input, _) = space1(input)?;
    let (input, delay) = u16(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::Release { id, pri, delay }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_release() {
        let input = "release 42 1 5\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            cmd,
            Command::Release {
                id: 42,
                pri: 1,
                delay: 5
            }
        );
    }
}
