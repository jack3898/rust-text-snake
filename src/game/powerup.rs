#[derive(PartialEq)]
pub enum PowerupType {
    Supersnake { duration: u64 },
    Slowdown { duration: u64 },
    None,
}
