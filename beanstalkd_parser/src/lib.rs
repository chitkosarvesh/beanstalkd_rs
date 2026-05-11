pub mod command;
mod parsers;
pub mod response;
use nom::{IResult, Parser, branch::alt};

use crate::command::Command;

pub fn parse_command(input: &str) -> IResult<&str, Command<'_>> {
    alt([
        parsers::put::parse_put,
        parsers::r#use::parse_use,
        parsers::reserve::parse_reserve,
        parsers::reserve::parse_reserve_with_timeout,
        parsers::delete::parse_delete,
        parsers::release::parse_release,
        parsers::bury::parse_bury,
        parsers::touch::parse_touch,
        parsers::watch::parse_watch,
        parsers::ignore::parse_ignore,
        parsers::peek::parse_peek,
        parsers::peek::parse_peek_ready,
        parsers::peek::parse_peek_delayed,
        parsers::peek::parse_peek_buried,
        parsers::kick::parse_kick,
        parsers::kick::parse_kick_job,
        parsers::stats::stats,
        parsers::stats::stats_job,
        parsers::stats::stats_tube,
        parsers::list::parse_list_tubes,
        parsers::list::list_tube_used,
        parsers::list::list_tubes_watched,
        parsers::quit::parse_quit,
        parsers::pause::parse_pause_tube,
    ])
    .parse(input)
}
