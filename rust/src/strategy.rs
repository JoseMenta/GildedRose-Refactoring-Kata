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


//Implement decorator using the same Trait (interface), saving the wrapped strategy
pub struct ConjuredStrategy{
    pub wrapped_strategy: Box<dyn Strategy>,
}

impl Strategy for ConjuredStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> i32 {
        if quality <= 0 || quality >= 50{
            return quality;
        }
        let obtained_quality = self.wrapped_strategy.execute(quality, sell_in);
        let delta_quality = 2 * (obtained_quality - quality);
        let final_quality = quality + delta_quality;
        if final_quality <= 0 {
            return 0;
        }
        if final_quality > 50{
            return 50;
        }
        final_quality
    }
}


pub struct BackstageTAFKALStrategy{}

impl Strategy for BackstageTAFKALStrategy {
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
pub struct BackStageGalaStrategy{}
impl Strategy for BackStageGalaStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> i32 {
        if quality <= 0 || quality >= 50{
            return quality;
        }
        if sell_in == 0{
            return 0;
        }
        let updated_quality = match sell_in{
            11.. => quality + 2,
            6..=10 => quality + 3,
            0..=5 => quality + 4,
            _ => quality + 1,
        };
        if updated_quality >= 50{
            return 50;
        }
        updated_quality
    }
}