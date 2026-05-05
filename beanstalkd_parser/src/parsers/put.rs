use nom::{
    IResult,
    bytes::complete::{tag, take},
    character::complete::{space1, u16},
};

use crate::command::Command;

pub fn parse_put(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("put")(input)?;
    let (input, _) = space1(input)?;
    let (input, pri) = u16(input)?;
    let (input, _) = space1(input)?;
    let (input, delay) = u16(input)?;
    let (input, _) = space1(input)?;
    let (input, ttr) = u16(input)?;
    let (input, _) = space1(input)?;
    let (input, bytes) = u16(input)?;
    let (input, _) = tag("\r\n")(input)?;
    let (input, data) = take(bytes as usize)(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((
        input,
        Command::Put {
            pri,
            delay,
            ttr,
            bytes,
            data,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::parse_command;

    use super::*;

    #[test]
    fn test_put() {
        let input = "put 1 0 30 11\r\nhello world\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(
            cmd,
            Command::Put {
                pri: 1,
                delay: 0,
                ttr: 30,
                bytes: 11,
                data: "hello world"
            }
        );
    }
    #[test]
    fn test_put_exact_bytes() {
        // `bytes` is consumed exactly; anything after the trailing \r\n stays in `remaining`
        let input = "put 0 0 60 5\r\nhello\r\nextra";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "extra");
        assert_eq!(
            cmd,
            Command::Put {
                pri: 0,
                delay: 0,
                ttr: 60,
                bytes: 5,
                data: "hello"
            }
        );
    }
}
