#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Boolean {
    always_true: bool,
    always_false: bool,
    variable: bool,
}
