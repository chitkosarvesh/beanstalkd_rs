#[derive(Debug, PartialEq)]
pub enum Command<'a> {
    Put {
        pri: u16,
        delay: u16,
        ttr: u16,
        bytes: u16,
        data: &'a str,
    },
    Use {
        tube: &'a str,
    },
    Reserve,
    ReserveWithTimeout {
        seconds: u16,
    },
    Delete {
        id: u64,
    },
    Release {
        id: u64,
        pri: u16,
        delay: u16,
    },
    Bury {
        id: u64,
        pri: u16,
    },
    Touch {
        id: u64,
    },
    Watch {
        tube: &'a str,
    },
    Ignore {
        tube: &'a str,
    },
    Peek {
        id: u64,
    },
    PeekReady,
    PeekDelayed,
    PeekBuried,
    Kick {
        bound: u16,
    },
    KickJob {
        id: u64,
    },
    StatsJob {
        id: u64,
    },
    StatsTube {
        tube: &'a str,
    },
    Stats,
    ListTubes,
    ListTubeUsed,
    ListTubesWatched,
    Quit,
    PauseTube {
        tube: &'a str,
        delay: u16,
    },
}
