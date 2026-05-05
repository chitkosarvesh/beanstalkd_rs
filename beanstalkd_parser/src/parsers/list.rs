use nom::{IResult, bytes::complete::tag};

use crate::command::Command;

pub fn parse_list_tubes(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("list-tubes\r\n")(input)?;
    Ok((input, Command::ListTubes))
}
pub fn list_tube_used(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("list-tube-used\r\n")(input)?;
    Ok((input, Command::ListTubeUsed))
}
pub fn list_tubes_watched(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("list-tubes-watched\r\n")(input)?;
    Ok((input, Command::ListTubesWatched))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_list_tubes() {
        let input = "list-tubes\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::ListTubes);
    }
    #[test]
    fn test_list_tube_used() {
        let input = "list-tube-used\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::ListTubeUsed);
    }
    #[test]
    fn test_list_tubes_watched() {
        let input = "list-tubes-watched\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::ListTubesWatched);
    }
}
