
macro_rules! update_sellin {
    ($sell_in:ident) => {
        if $sell_in == 0 { 0 } else { $sell_in - 1 }
    };
}
pub trait Strategy{
    fn execute(&self, quality: i32, sell_in: i32) -> (i32, i32);
}

pub struct BrieStrategy{}

impl Strategy for BrieStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> (i32, i32) {
        if quality <= 0 || quality >= 50{
            return (quality ,update_sellin!(sell_in));
        }
        if sell_in <= 0 && quality <= 48{
            return (quality + 2,update_sellin!(sell_in));
        }
        (quality + 1,update_sellin!(sell_in))
    }
}

pub struct StandardStrategy{}

impl Strategy for StandardStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> (i32,i32) {
        if quality <= 0 || quality >= 50{
            return (quality,update_sellin!(sell_in));
        }
        if sell_in <= 0 && quality >= 2{
            return (quality - 2,update_sellin!(sell_in));
        }
        (quality - 1,update_sellin!(sell_in))
    }
}

pub struct SulfurasStrategy{}

impl Strategy for SulfurasStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> (i32,i32) {
        (quality,sell_in)
    }
}


//Implement decorator using the same Trait (interface), saving the wrapped strategy
pub struct ConjuredStrategy{
    pub wrapped_strategy: Box<dyn Strategy>,
}

impl Strategy for ConjuredStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> (i32,i32) {
        let (obtained_quality,obtained_selllin) = self.wrapped_strategy.execute(quality, sell_in);
        let delta_quality = 2 * (obtained_quality - quality);
        let final_quality = quality + delta_quality;
        if final_quality <= 0 {
            return (0,obtained_selllin);
        }
        if final_quality > 50{
            return (final_quality,obtained_selllin);
        }
        (final_quality,obtained_selllin)
    }
}


pub struct BackstageTAFKALStrategy{}

impl Strategy for BackstageTAFKALStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> (i32,i32) {
        if quality <= 0 || quality >= 50{
            return (quality,update_sellin!(sell_in));
        }
        if sell_in == 0{
            return (0,update_sellin!(sell_in));
        }
        let updated_quality = match sell_in{
            6..=10 => quality + 2,
            0..=5 => quality + 3,
            _ => quality + 1,
        };
        if updated_quality >= 50{
            return (50,update_sellin!(sell_in));
        }
        (updated_quality,update_sellin!(sell_in))
    }
}
pub struct BackStageGalaStrategy{}
impl Strategy for BackStageGalaStrategy {
    fn execute(&self, quality: i32, sell_in: i32) -> (i32,i32) {
        if quality <= 0 || quality >= 50{
            return (quality,update_sellin!(sell_in));
        }
        if sell_in == 0{
            return (0,update_sellin!(sell_in));
        }
        let updated_quality = match sell_in{
            11.. => quality + 2,
            6..=10 => quality + 3,
            0..=5 => quality + 4,
            _ => quality + 1, //unreachable
        };
        if updated_quality >= 50{
            return (50,update_sellin!(sell_in));
        }
        (updated_quality,update_sellin!(sell_in))
    }
}