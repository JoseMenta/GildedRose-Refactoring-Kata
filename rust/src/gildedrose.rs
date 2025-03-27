use crate::item::Item;
use crate::strategy::{BackStageGalaStrategy, BackstageTAFKALStrategy, BrieStrategy, ConjuredStrategy, StandardStrategy, Strategy, SulfurasStrategy};

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in &mut self.items {
            let mut selected_strategy: Box<dyn Strategy> =
            if item.name.contains("Sulfuras"){
                Box::new(SulfurasStrategy{})
            }else if  item.name.contains("Aged Brie"){
                Box::new(BrieStrategy{})
            }else if  item.name.contains("Backstage passes to a TAFKAL80ETC"){
                Box::new(BackstageTAFKALStrategy{})
            }else if item.name.contains("Backstage passes to a GALA"){
                Box::new(BackStageGalaStrategy{})
            } else{
                Box::new(StandardStrategy{})
            };
            if item.name.contains("Conjured"){
                selected_strategy = Box::new(ConjuredStrategy{wrapped_strategy: selected_strategy})
            }
            item.set_strategy(Some(selected_strategy));
            item.update_quality()
        }
    }
}