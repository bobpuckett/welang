use crate::flattener::Step;

pub trait Construct {
    fn flatten() -> Step;
}
