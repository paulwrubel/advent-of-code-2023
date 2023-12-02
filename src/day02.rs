use crate::{AdventError, ExclusivePart};

pub fn run(epart: ExclusivePart) -> Result<String, AdventError> {
    match epart {
        ExclusivePart::One => part_one(),
        ExclusivePart::Two => part_two(),
    }
}

fn part_one() -> Result<String, AdventError> {
    Err(AdventError::Unimplemented)
}

fn part_two() -> Result<String, AdventError> {
    Err(AdventError::Unimplemented)
}
