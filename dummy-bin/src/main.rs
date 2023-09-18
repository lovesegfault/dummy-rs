use std::{error::Error, fmt};

use dummy_lib::times_two;

enum DummyError {
    NoArg,
    Parse(std::num::ParseIntError),
}

impl fmt::Debug for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl fmt::Display for DummyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoArg => write!(f, "No argument provided, expected NUM"),
            Self::Parse(e) => write!(f, "Failed to parse provided NUM: {e}"),
        }
    }
}

impl Error for DummyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoArg => None,
            Self::Parse(source) => Some(source),
        }
    }
}

fn main() -> Result<(), DummyError> {
    let mut args = std::env::args();

    let _bin_name = args.next();

    let Some(num) = args.next() else {
        return Err(DummyError::NoArg);
    };

    let num: i32 = num.parse().map_err(|e| DummyError::Parse(e))?;

    let num_two = times_two(num);

    println!("dummy-bin: {num} * 2 = {num_two}");

    Ok(())
}
