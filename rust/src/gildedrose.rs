use std::fmt::{self, Display};
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

pub trait Strategy{
    fn execute(&self, quality: i32, sell_in: i32) -> i32;
}

struct BrieStrategy{}

impl Strategy for BrieStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> i32 {
        if quality <= 0 || quality >= 50{
            return quality
        }
        if sell_in <= 0 && quality <= 48{
            return quality + 2;
        }
        quality + 1
    }
}

struct StandardStrategy{}

impl Strategy for StandardStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> i32 {
        if quality <= 0 || quality >= 50{
            return quality;
        }
        if sell_in <= 0 && quality >= 2{
            return quality - 2;
        }
        quality - 1
    }
}

struct SulfurasStrategy{}

impl Strategy for SulfurasStrategy {
    fn execute(&self, quality: i32, _: i32) -> i32 {
        quality
    }
}

struct ConjuredStrategy{}

impl Strategy for ConjuredStrategy {
    fn execute(&self, quality: i32, _: i32) -> i32 {
        if quality <= 0 || quality >= 50{
            return quality;
        }
        quality - 2
    }
}


struct BackstageStrategy{}

impl Strategy for BackstageStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> i32 {
        if quality <= 0 || quality >= 50{
            return quality;
        }
        if sell_in == 0{
            return 0;
        }
        let updated_quality = match sell_in{
            6..=10 => quality + 2,
            0..=5 => quality + 3,
            _ => quality + 1,
        };
        if updated_quality >= 50{
            return 50;
        }
        updated_quality
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }


    // Todos tienen SellIn y Quality
    // En general, Quality esta entre 0 y 50
    // Una vez que pasa SellIn, Quality se degrada al doble de velocidad
    // Aged Brie aumenta su Quality con el tiempo
    // Sulfuras Nunca tiene SellIn o Quality
    // Backstage passes también aumenta Quality a medida que se acerca SellIn
    // -Aumenta 2 cuando faltan 10 días o menos, 3 cuando faltan 5 o menos
    // -Es 0 cuando sellin==0
    // Conjured bajan su calidad el doble de rápido
    // Sulfuras siempre tiene quality en 80
    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            if  self.items[i].name.contains("Sulfuras"){
                self.items[i].set_strategy(Some(Box::new(SulfurasStrategy{})))
            }else if  self.items[i].name.contains("Aged Brie"){
                self.items[i].set_strategy(Some(Box::new(BrieStrategy{})))
            }else if  self.items[i].name.contains("Backstage passes"){
                self.items[i].set_strategy(Some(Box::new(BackstageStrategy{})))
            }else if self.items[i].name.contains("Conjured"){
                self.items[i].set_strategy(Some(Box::new(ConjuredStrategy{})))
            }else{
                self.items[i].set_strategy(Some(Box::new(StandardStrategy{})))
            }
            self.items[i].update_quality()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};


    #[test]
    pub fn test_aged_brie_quality_limits(){
        let items = vec![Item::new("Aged Brie", 2, 50)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(50, rose.items[0].quality);
    }
    #[test]
    pub fn test_aged_brie_quality_increase_doble(){
        let items = vec![Item::new("Aged Brie", 0, 40)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(42, rose.items[0].quality);
    }
    #[test]
    pub fn test_aged_brie_quality_increase(){
        let items = vec![Item::new("Aged Brie", 2, 40)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(41, rose.items[0].quality);
    }
    #[test]
    pub fn test_sulfuras_quality(){
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 2, 80)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(80, rose.items[0].quality);
    }
    #[test]
    pub fn test_sulfuras_sellin(){
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 2, 80)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(2, rose.items[0].sell_in);
    }
    #[test]
    pub fn test_backstage_passes_quality(){
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 15, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(21, rose.items[0].quality);
    }
    #[test]
    pub fn test_backstage_passes_quality_10(){
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(22, rose.items[0].quality);
    }
    #[test]
    pub fn test_backstage_passes_quality_5(){
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(23, rose.items[0].quality);
    }
    #[test]
    pub fn test_backstage_passes_quality_0(){
        let items = vec![Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(0, rose.items[0].quality);
    }
    #[test]
    pub fn test_normal_item_quality(){
        let items = vec![Item::new("foo", 2, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(19, rose.items[0].quality);
    }
    #[test]
    pub fn test_normal_after_sellin(){
        let items = vec![Item::new("foo", 0, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(18, rose.items[0].quality);
    }
}
