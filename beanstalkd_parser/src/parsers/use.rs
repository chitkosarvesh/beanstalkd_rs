use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::space1,
};

use crate::command::Command;

pub fn parse_use(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("use")(input)?;
    let (input, _) = space1(input)?;
    let (input, tube) = take_until("\r\n")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::Use { tube }))
}

#[cfg(test)]
mod tests {
    use crate::parse_command;

    use super::*;
    #[test]
    fn test_use() {
        let input = "use my-tube\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Use { tube: "my-tube" });
    }
}
