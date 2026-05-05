use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{space1, u64},
};

use crate::command::Command;

pub fn stats(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("stats\r\n")(input)?;
    Ok((input, Command::Stats))
}
pub fn stats_job(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("stats-job")(input)?;
    let (input, _) = space1(input)?;
    let (input, id) = u64(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::StatsJob { id }))
}
pub fn stats_tube(input: &str) -> IResult<&str, Command<'_>> {
    let (input, _) = tag("stats-tube")(input)?;
    let (input, _) = space1(input)?;
    let (input, tube) = take_until("\r\n")(input)?;
    let (input, _) = tag("\r\n")(input)?;
    Ok((input, Command::StatsTube { tube }))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_command;
    #[test]
    fn test_stats() {
        let input = "stats\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::Stats);
    }
    #[test]
    fn test_stats_job() {
        let input = "stats-job 42\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::StatsJob { id: 42 });
    }
    #[test]
    fn test_stats_tube() {
        let input = "stats-tube my-tube\r\n";
        let (remaining, cmd) = parse_command(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(cmd, Command::StatsTube { tube: "my-tube" });
    }
}
