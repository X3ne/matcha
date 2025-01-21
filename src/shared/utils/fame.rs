use std::cmp;

pub struct FameCalculator;

impl FameCalculator {
    pub fn calculate_fame_multiplier(liker_fame: i32) -> f64 {
        1.0 + ((liker_fame as f64).log2() / 10.0)
    }

    pub fn calculate_fame(current_fame: i32, fame_multiplier: f64) -> i32 {
        let base_fame = 1;
        let scale_factor = 5;

        let fame = ((base_fame as f64 + (scale_factor as f64 / (1.0 + (current_fame as f64).log2()))) * fame_multiplier)
            as i32;

        cmp::max(fame, 1)
    }
}
