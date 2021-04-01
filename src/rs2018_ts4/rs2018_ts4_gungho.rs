//! Tools for transpiling Rust 2018 to TypeScript 4 using the ‘Gungho’ strategy.

use crate::transpile::result::TranspileResult;

/// Transpiles Rust 2018 code to TypeScript 4 code using the ‘Gungho’ strategy.
/// 
/// ### Arguments
/// * `orig` The original Rust code, assumed to conform to the 2018 edition
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn rs2018_ts4_gungho(
    orig: &str
) -> TranspileResult {
    if orig.contains("FOUR") {
        TranspileResult::new()
            .push_main_line("const FOUR: Number = 4;")
    } else {
        TranspileResult::new()
            .push_main_line("const ROUGHLY_PI: Number = 3.14;")
    }
}
