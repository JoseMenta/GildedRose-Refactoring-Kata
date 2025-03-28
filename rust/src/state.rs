macro_rules! update_sell_in {
    ($sell_in:ident) => {
        if $sell_in == 0 { 0 } else { $sell_in - 1 }
    };
}

const STANDARD_QUALITY_UPPER_LIMIT: i32 = 50;
const STANDARD_QUALITY_LOWER_LIMIT: i32 = 0;
macro_rules! standard_quality {
    ($quality:expr) => {
        std::cmp::max(std::cmp::min($quality, STANDARD_QUALITY_UPPER_LIMIT), STANDARD_QUALITY_LOWER_LIMIT)
    };
}


// Trait for cloning State objects
pub trait StateClone {
    fn clone_box(&self) -> Box<dyn State>;
}

// Implement Clone for Box<dyn State>
impl Clone for Box<dyn State> {
    fn clone(&self) -> Box<dyn State> {
        self.clone_box()
    }
}

//Implementing State requires implementing StateClone ('static means it does not have a reference shorter than static)
pub trait State: StateClone + 'static {
    fn update(&self, quality: i32, sell_in: i32) -> (i32, i32);
    fn next_state(&self, sell_in: i32) -> Option<Box<dyn State>>;
}

impl <T: State + Clone> StateClone for T {
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub struct StandardState{
    pub quality_delta: i32,
    pub sell_in_limit: i32,
    pub next_state: Box<dyn State>,
}

impl State for StandardState {
    fn update(&self, quality: i32, sell_in: i32) -> (i32, i32) {
        (standard_quality!(quality + self.quality_delta), update_sell_in!(sell_in))
    }

    fn next_state(&self, sell_in: i32) -> Option<Box<dyn State>> {
        if sell_in <= self.sell_in_limit{
            Some(self.next_state.clone()) // Clone the next_state
        }else{
            None
        }
    }
}

#[derive(Clone)]
pub struct ConjuredState{
    pub wrapped_state: Box<dyn State>,
}


impl State for ConjuredState {

    fn update(&self, quality: i32, sell_in: i32) -> (i32, i32) {
        let (obtained_quality, obtained_sell_in) = self.wrapped_state.update(quality, sell_in);
        let final_quality = quality + 2*(obtained_quality - quality);
        if final_quality <= STANDARD_QUALITY_LOWER_LIMIT{
            return (STANDARD_QUALITY_LOWER_LIMIT, obtained_sell_in);
        }
        if final_quality >= STANDARD_QUALITY_UPPER_LIMIT{
            return (obtained_quality, obtained_sell_in);
        }
        (final_quality, obtained_sell_in)

    }

    fn next_state(&self, sell_in: i32) -> Option<Box<dyn State>> {
        if let Some(wrapped_state) = self.wrapped_state.next_state(sell_in){
            Some(Box::new(ConjuredState{wrapped_state}))
        }else{
            None
        }
    }
}

#[derive(Clone)]
pub struct ConstantState{
    pub quality: i32
}

impl State for ConstantState {
    fn update(&self, _: i32, sell_in: i32) -> (i32, i32) {
        (self.quality, sell_in)
    }

    fn next_state(&self, _: i32) -> Option<Box<dyn State>> {
        None
    }
}



#[derive(Clone)]
pub struct StableState{}


impl State for StableState {
    fn update(&self, quality: i32, sell_in: i32) -> (i32, i32) {
        (quality, sell_in)
    }

    fn next_state(&self, _: i32) -> Option<Box<dyn State>> {
        None
    }
}

