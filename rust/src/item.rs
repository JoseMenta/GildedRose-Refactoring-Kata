use std::fmt;
use std::fmt::Display;
use crate::state::{ConjuredState, ConstantState, StableState, StandardState, State};
fn get_surfuras_state_machine() -> Box<dyn State> {
    Box::new(StableState{})
}

fn get_aged_brie_state_machine(sell_in: i32) -> Box<dyn State> {
    let passed_state = Box::new(StandardState{
        sell_in_limit: -1,
        quality_delta: 2,
        next_state: Box::new(StableState{})
    });
    if sell_in <= 0{
        return passed_state;
    }
    Box::new(StandardState{
        sell_in_limit: 0,
        quality_delta: 1,
        next_state: passed_state
    })
}

fn get_standard_state_machine(sell_in: i32) -> Box<dyn State> {
    let passed_state = Box::new(StandardState{
        sell_in_limit: -1,
        quality_delta: -2,
        next_state: Box::new(StableState{})
    });
    if sell_in <= 0{
        return passed_state
    }
    Box::new(StandardState{
        sell_in_limit: 0,
        quality_delta: -1,
        next_state: passed_state
    })
}

fn get_conjured_state_machine(wrapped_state: Box<dyn State>) -> Box<dyn State> {
    Box::new(ConjuredState{wrapped_state})
}

fn get_backstage_tafkal_state_machine(sell_in: i32) -> Box<dyn State> {
    let passed_state = Box::new(ConstantState{quality:0});
    if sell_in <= 0{
        return passed_state;
    }
    let until_zero_state = Box::new(StandardState{
        quality_delta: 3,
        sell_in_limit: 0,
        next_state: passed_state
    });
    if sell_in <= 5{
        return until_zero_state;
    }
    let until_five_state = Box::new(StandardState{
        quality_delta: 2,
        sell_in_limit: 5,
        next_state: until_zero_state
    });
    if sell_in <= 10{
        return until_five_state;
    }
    Box::new(StandardState{
        quality_delta: 1,
        sell_in_limit: 10,
        next_state: until_five_state
    })
}

fn get_backstage_gala_state_machine(sell_in: i32) -> Box<dyn State> {
    let passed_state = Box::new(ConstantState{quality:0});
    if sell_in <= 0{
        return passed_state;
    }
    let until_zero_state = Box::new(StandardState{
        quality_delta: 4,
        sell_in_limit: 0,
        next_state: passed_state
    });
    if sell_in <= 5{
        return until_zero_state;
    }
    let until_five_state = Box::new(StandardState{
        quality_delta: 3,
        sell_in_limit: 5,
        next_state: until_zero_state
    });
    if sell_in <= 10{
        return until_five_state;
    }
    Box::new(StandardState{
        quality_delta: 2,
        sell_in_limit: 10,
        next_state: until_five_state
    })
}

fn get_item_state_machine(name: &str, sell_in: i32) -> Box<dyn State> {
    let mut state: Box<dyn State> =
        if name.contains("Sulfuras"){
            get_surfuras_state_machine()
        }else if  name.contains("Aged Brie"){
            get_aged_brie_state_machine(sell_in)
        }else if  name.contains("Backstage passes to a TAFKAL80ETC"){
            get_backstage_tafkal_state_machine(sell_in)
        }else if name.contains("Backstage passes to a GALA"){
            get_backstage_gala_state_machine(sell_in)
        } else{
            get_standard_state_machine(sell_in)
        };
    if name.contains("Conjured"){
        state = get_conjured_state_machine(state);
    }
    state
}

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
    pub state: Box<dyn State>,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        let name = name.into();
        let state = get_item_state_machine(&name, sell_in);
        Item {
            name,
            sell_in,
            quality,
            state
        }
    }
    pub fn update_quality(&mut self){
        (self.quality, self.sell_in) = self.state.update(self.quality, self.sell_in);
        if let Some(state) = self.state.next_state(self.sell_in){
            self.state = state;
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}