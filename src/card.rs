use crate::unit::Energy;

pub enum CardType {
    Attack,
    Skill,
    Power,
    Status,
    Curse,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub enum CardName {
    Strike,
    Defend,
}

#[derive(PartialEq, Debug, Clone, Copy, Eq, PartialOrd, Ord)]
pub struct Card {
    pub name: CardName,
    pub cost: Energy,
}

impl Card {
    pub fn get_type(&self) -> CardType {
        match self.name {
            CardName::Strike => CardType::Attack,
            CardName::Defend => CardType::Skill,
        }
    }
}
