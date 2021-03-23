//! Contains the library’s main function, `rs_to_ts()`.

use super::config::{Config,RsEdition,Strategy,TsMajor};
use super::result::TranspileResult;

/// Transpiles Rust code to TypeScript.
/// 
/// This is the library’s main function.
/// 
/// ### Arguments
/// * `raw` The original Rust code
/// * `config` Defines code versions and transpilation strategy — see below
/// 
/// ### Returns
/// @TODO document what this function returns
/// 
/// ### The `config` argument
/// For default configuration, just pass in `Config::new()`.
/// ```
/// # use opinionated_rust_to_typescript::transpile::config::Config;
/// # use opinionated_rust_to_typescript::transpile::transpile::rs_to_ts;
/// assert_eq!(rs_to_ts(
///     "const ROUGHLY_PI: f32 = 3.14;",
///     Config::new()).main_lines[0],
///     "const ROUGHLY_PI: Number = 3.14;");
/// ```
/// The Builder Pattern lets you can modify your `Config` quite easily, and you
/// can use `to_string()` to inspect it. See the [Config] docs.
/// 
/// ### Placeholder config
/// Currently `rs_to_ts()` only supports input code in the 2018 edition of Rust,
/// and will only output TypeScript 4 code using the ‘Gungho’ strategy. The
/// following enum values are placeholders, and may be implementated one day:
/// * `RsEdition::Rs2015`
/// * `Strategy::Cautious`
/// * `TsMajor::Ts3`
/// 
/// Attempting to use placeholder config values leads to an error.
/// ```
/// # use opinionated_rust_to_typescript::transpile::config::*;
/// # use opinionated_rust_to_typescript::transpile::transpile::*;
/// assert_eq!(rs_to_ts("Nope",
///     Config::new().rs_edition(RsEdition::Rs2015)).errors[0].message,
///     "RsEdition::Rs2015 is not implemented yet");
/// assert_eq!(rs_to_ts("Nope",
///     Config::new().strategy(Strategy::Cautious)).errors[0].message,
///     "Strategy::Cautious is not implemented yet");
/// assert_eq!(rs_to_ts("Nope",
///     Config::new().ts_major(TsMajor::Ts3)).errors[0].message,
///     "TsMajor::Ts3 is not implemented yet");
/// ```
/// 
pub fn rs_to_ts(
    raw: &str,
    config: Config,
) -> TranspileResult {
    if config.rs_edition == RsEdition::Rs2015 {
        return make_not_implemented_result(
            "RsEdition::Rs2015 is not implemented yet");
    }
    if config.strategy == Strategy::Cautious {
        return make_not_implemented_result(
            "Strategy::Cautious is not implemented yet");
    }
    if config.ts_major == TsMajor::Ts3 {
        return make_not_implemented_result(
            "TsMajor::Ts3 is not implemented yet");
    }
    crate::rs2018_ts4::rs2018_ts4_gungho::rs2018_ts4_gungho(raw)
}

fn make_not_implemented_result(message: &'static str) -> TranspileResult {
    TranspileResult::new()
        .push_config_not_implemented_error(0, 0, message)
}