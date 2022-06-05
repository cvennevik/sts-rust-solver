#[derive(PartialEq, Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Energy {
    amount: i32,
}

impl Energy {
    pub fn from(amount: i32) -> Energy {
        Energy { amount: amount }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Health(pub i32);

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Armor(pub i32);
