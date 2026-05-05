pub enum Response {
    Inserted { id: u64 },
    Buried { id: u64 },
    ExpectedCRLF,
    JobTooBig,
    Draining,
    Using { tube: String },
    Reserved { id: u64, bytes: usize },
    Found { id: u64, bytes: u64 },
    DeadlineSoon,
    TimedOut,
    Deleted,
    NotFound,
    Released,
    Touched,
    Watching { count: u16 },
    Kicked,
    Ok { count: u16 },
    Paused,
}

pub fn convert_response(response: Response) -> String {
    match response {
        Response::Inserted { id } => format!("INSERTED {}", id),
        Response::Buried { id } => format!("BURIED {}", id),
        Response::ExpectedCRLF => "EXPECTED_CRLF".into(),
        Response::JobTooBig => "JOB_TOO_BIG".into(),
        Response::Draining => "DRAINING".into(),
        Response::Using { tube } => format!("USING {}", tube),
        Response::Reserved { id, bytes } => format!("RESERVED {} {}", id, bytes),
        Response::Found { id, bytes } => format!("FOUND {} {}", id, bytes),
        Response::DeadlineSoon => "DEADLINE_SOON".into(),
        Response::TimedOut => "TIMED_OUT".into(),
        Response::Deleted => "DELETED".into(),
        Response::NotFound => "NOT_FOUND".into(),
        Response::Released => "RELEASED".into(),
        Response::Touched => "TOUCHED".into(),
        Response::Watching { count } => format!("WATCHING {}", count),
        Response::Kicked => "KICKED".into(),
        Response::Ok { count } => format!("OK {}", count),
        Response::Paused => "PAUSED".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_response() {
        let response = Response::Inserted { id: 42 };
        assert_eq!(convert_response(response), "INSERTED 42");
        let response = Response::Using {
            tube: "my-tube".into(),
        };
        assert_eq!(convert_response(response), "USING my-tube");
        let response = Response::Reserved { id: 42, bytes: 100 };
        assert_eq!(convert_response(response), "RESERVED 42 100");
        let response = Response::Watching { count: 5 };
        assert_eq!(convert_response(response), "WATCHING 5");
    }
}
