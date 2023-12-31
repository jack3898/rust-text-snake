#[derive(PartialEq)]
pub enum PowerupType {
    Supersnake { tick_duration: u64 },
    Slowdown { tick_duration: u64 },
    None,
}
