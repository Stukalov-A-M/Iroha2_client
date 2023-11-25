use std::num::NonZeroU64;

pub fn non_zero_handler(number: u64) -> NonZeroU64 {
    NonZeroU64::new(number).map_or_else(
        || {
            println!("The number must be > 0, using default value 1");
            NonZeroU64::new(1).unwrap()
        },
        |non_zero| non_zero,
    )
}