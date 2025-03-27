#[cfg(test)]
mod tests {
    use crate::gildedrose::GildedRose;
    use crate::item::Item;

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
    pub fn test_aged_brie_sellin(){
        let items = vec![Item::new("Aged Brie", 2, 40)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(1, rose.items[0].sell_in);
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
    #[test]
    pub fn test_sellin(){
        let items = vec![Item::new("foo", 2, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(1, rose.items[0].sell_in);
    }

    #[test]
    pub fn test_backstage_gala_passes_quality_11(){
        let items = vec![Item::new("Backstage passes to a GALA concert", 11, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(22, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_gala_passes_quality_10(){
        let items = vec![Item::new("Backstage passes to a GALA concert", 10, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(23, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_gala_passes_quality_6(){
        let items = vec![Item::new("Backstage passes to a GALA concert", 6, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(23, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_gala_passes_quality_5(){
        let items = vec![Item::new("Backstage passes to a GALA concert", 5, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(24, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_gala_passes_quality_1(){
        let items = vec![Item::new("Backstage passes to a GALA concert", 1, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(24, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_gala_passes_quality_0(){
        let items = vec![Item::new("Backstage passes to a GALA concert", 0, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(0, rose.items[0].quality);
    }

    #[test]
    pub fn test_conjured_quality(){
        let items = vec![Item::new("Conjured Mana Cake", 3, 6)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(4, rose.items[0].quality);
    }

    #[test]
    pub fn test_backstage_gala_passes_sellin(){
        let items = vec![Item::new("Backstage passes to a GALA concert", 5, 20)];
        let mut rose = GildedRose::new(items);
        rose.update_quality();
        assert_eq!(4, rose.items[0].sell_in);
    }
}
