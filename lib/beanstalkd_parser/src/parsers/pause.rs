use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{space1, u16},
};

use crate::command::Command;

pub fn parse_pause_tube(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("pause-tube")(input)?;
    let (input, _) = space1(input)?;
    let (input, tube) = take_until(" ")(input)?;
    let (input, _) = space1(input)?;
    let (input, delay) = u16(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::PauseTube { tube, delay }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_pause_tube() {
        let input = "pause-tube my-tube 5\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            cmd,
            Command::PauseTube {
                tube: "my-tube",
                delay: 5
            }
        );
    }
}
