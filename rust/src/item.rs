use std::fmt;
use std::fmt::Display;
use crate::strategy::Strategy;

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
    pub strategy: Option<Box<dyn Strategy>>
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
            strategy: None
        }
    }
    pub fn set_strategy(&mut self, strategy: Option<Box<dyn Strategy>>) {
        self.strategy = strategy;
    }
    pub fn update_quality(&mut self){
        if let Some(strategy) = &self.strategy  {
            self.quality = strategy.execute(self.quality, self.sell_in);
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}