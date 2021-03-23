//! Contains the library’s main function, `rs_to_ts()`.

use super::transpile_result::TranspileResult;

/// The edition of Rust that the input code is written in.
#[derive(PartialEq)]
pub enum RsEdition {
    /// The most recent Rust edition that this library supports.
    Latest,
    /// _`Rs2015` is a placeholder. This edition is currently not supported._
    Rs2015,
    /// Currently, only the 2018 edition of Rust is supported.
    Rs2018,
}

/// Which strategy to use when transpiling Rust code into TypeScript.
#[derive(PartialEq)]
pub enum Strategy {
    /// __Favours safety over readability.__
    /// 
    /// A strategy which is quite verbose, and looks very different to the input
    /// Rust code, but does not pollute global scope.
    /// 
    /// _`Cautious` is a placeholder. This strategy is currently not supported._
    Cautious,
    /// __Favours readability over safety.__
    ///
    /// Pollutes global scope by adding
    /// methods to native `prototype` objects, eg `String.prototype.len()`.
    /// Looks very similar to the input Rust code, and attempts to preserve line
    /// numbers.
    /// 
    /// _Currently the only strategy which `rs_to_ts()` supports._
    Gungho,
}

/// The major version of TypeScript that `rs_to_ts` should output.
#[derive(PartialEq)]
pub enum TsMajor {
    /// The most recent TypeScript major-version that this library supports.
    Latest,
    /// _`Ts3` is a placeholder. This version is currently not supported._
    Ts3,
    /// Currently, only TypeScript 4 is supported.
    Ts4,
}

/// A configuration object. Controls how the Rust code should be transpiled
/// to TypeScript.
/// 
/// ### The `to_string()` method
/// `Config::to_string()` provides a handy summary of your confguration. In this
/// case, `rs_to_ts()` will expect the `raw` argument to be 2018 edition Rust,
/// and will output very readable TypeScript 4, which pollutes global scope.
/// ```
/// # use opinionated_rust_to_typescript::rs_ts::rs_ts::Config;
/// assert_eq!(Config::new().to_string(),
///     "Latest Rust edition (2018), Latest TypeScript (4), Gungho");
/// ```
/// 
/// ### Modifying `Config`
/// Use `rs_edition()`, `strategy()` and `ts_major()` to set the parameters.
/// ```
/// # use opinionated_rust_to_typescript::rs_ts::rs_ts::*;
/// assert_eq!(Config::new().rs_edition(RsEdition::Rs2015).to_string(),
///     "Rust edition 2015, Latest TypeScript (4), Gungho");
/// assert_eq!(Config::new().strategy(Strategy::Cautious).to_string(),
///     "Latest Rust edition (2018), Latest TypeScript (4), Cautious");
/// assert_eq!(Config::new().ts_major(TsMajor::Ts3).to_string(),
///     "Latest Rust edition (2018), TypeScript 3, Gungho");
/// assert_eq!(Config::new()
/// .strategy(Strategy::Cautious)
/// .rs_edition(RsEdition::Rs2015)
/// .ts_major(TsMajor::Ts3)
/// .rs_edition(RsEdition::Rs2018)
/// .ts_major(TsMajor::Ts4)
/// .to_string(),
///     "Rust edition 2018, TypeScript 4, Cautious");
/// ```
/// 
/// ### The Builder Pattern
/// 
/// For more information about the Builder Pattern:
/// <https://doc.rust-lang.org/1.0.0/style/ownership/builders.html>
/// ///
pub struct Config {
    rs_edition: RsEdition,
    strategy: Strategy,
    ts_major: TsMajor,
}

impl Config {
    /// Creates a default Config object, to pass to `rs_to_ts()`.
    pub fn new() -> Self {
        Config {
            rs_edition: RsEdition::Latest,
            strategy: Strategy::Gungho,
            ts_major: TsMajor::Latest,
        }
    }
    /// Overrides the configuration’s default ‘Rust edition’.
    pub fn rs_edition(mut self, replacement_value: RsEdition) -> Self {
        self.rs_edition = replacement_value;
        return self;
    }
    /// Overrides the configuration’s default transpilation strategy.
    pub fn strategy(mut self, replacement_value: Strategy) -> Self {
        self.strategy = replacement_value;
        return self;
    }
    /// Overrides the configuration’s default ‘TypeScript major-version’.
    pub fn ts_major(mut self, replacement_value: TsMajor) -> Self {
        self.ts_major = replacement_value;
        return self;
    }
    /// Displays the configuration in a human-readable CSV format.
    pub fn to_string(&self) -> String {
        let mut out: String = "".into();
        out.push_str(match &self.rs_edition {
            RsEdition::Latest => "Latest Rust edition (2018), ",
            RsEdition::Rs2015 => "Rust edition 2015, ",
            RsEdition::Rs2018 => "Rust edition 2018, ",
        }.into());
        out.push_str(match &self.ts_major {
            TsMajor::Latest => "Latest TypeScript (4), ",
            TsMajor::Ts3 => "TypeScript 3, ",
            TsMajor::Ts4 => "TypeScript 4, ",
        }.into());
        out.push_str(match &self.strategy {
            Strategy::Cautious => "Cautious",
            Strategy::Gungho => "Gungho",
        }.into());
        return out;
    }
}


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
/// # use opinionated_rust_to_typescript::rs_ts::rs_ts::{Config,rs_to_ts};
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
/// # use opinionated_rust_to_typescript::rs_ts::rs_ts::*;
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