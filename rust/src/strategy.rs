pub trait Strategy{
    fn execute(&self, quality: i32, sell_in: i32) -> i32;
}

pub struct BrieStrategy{}

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

pub struct StandardStrategy{}

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

pub struct SulfurasStrategy{}

impl Strategy for SulfurasStrategy {
    fn execute(&self, quality: i32, _: i32) -> i32 {
        quality
    }
}

pub struct ConjuredStrategy{}

impl Strategy for ConjuredStrategy {
    fn execute(&self, quality: i32, _: i32) -> i32 {
        if quality <= 0 || quality >= 50{
            return quality;
        }
        quality - 2
    }
}


pub struct BackstageStrategy{}

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