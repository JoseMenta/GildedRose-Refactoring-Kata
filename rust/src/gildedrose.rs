use crate::item::Item;
use crate::strategy::{BackstageStrategy, BrieStrategy, ConjuredStrategy, StandardStrategy, SulfurasStrategy};

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            if item.name.contains("Sulfuras"){
                item.set_strategy(Some(Box::new(SulfurasStrategy{})))
            }else if  item.name.contains("Aged Brie"){
                item.set_strategy(Some(Box::new(BrieStrategy{})))
            }else if  item.name.contains("Backstage passes"){
                item.set_strategy(Some(Box::new(BackstageStrategy{})))
            }else if item.name.contains("Conjured"){
                item.set_strategy(Some(Box::new(ConjuredStrategy{})))
            }else{
                item.set_strategy(Some(Box::new(StandardStrategy{})))
            }
            item.update_quality()
        }
    }
}