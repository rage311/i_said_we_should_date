use libc;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::DateTime;

const ENV_VAR: &str = "DT_OVERRIDE";

fn now_epoch() -> libc::time_t {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as libc::time_t
}

#[no_mangle]
pub unsafe extern "C" fn time() -> libc::time_t {
    if let Ok(timestamp) = std::env::var(ENV_VAR) {
        if let Ok(parsed) = timestamp.parse::<libc::time_t>() {
            return parsed;
        } else if let Ok(parsed) = DateTime::parse_from_rfc3339(&timestamp) {
            return parsed.timestamp();
        }
        eprintln!("Unable to parse '{}' as epoch or RFC3339", timestamp);
    }

    now_epoch()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn rfc3339_fail() {
        std::env::set_var(ENV_VAR, "2009-02-13 23:31:30");
        assert_eq!(unsafe { time() }, now_epoch());
    }

    #[test]
    fn rfc3339_success() {
        std::env::set_var(ENV_VAR, "2009-02-13T23:31:30Z");
        assert_eq!(unsafe { time() }, 1234567890);
    }

    #[test]
    fn epoch_fail() {
        std::env::set_var(ENV_VAR, "the fifth of november");
        assert_eq!(unsafe { time() }, now_epoch());
    }

    #[test]
    fn epoch_success() {
        std::env::set_var(ENV_VAR, "1234567890");
        assert_eq!(unsafe { time() }, 1234567890);
    }

    #[test]
    fn time_default() {
        std::env::remove_var(ENV_VAR);
        assert_eq!(unsafe { time() }, now_epoch());
    }
}
