//! A configuration object which controls how Rust is transpiled to TypeScript.

/// A configuration object which controls how Rust is transpiled to TypeScript.
/// 
/// ### The `to_string()` method
/// `Config::to_string()` provides a handy summary of your confguration. In this
/// case, `rs_to_ts()` will expect the `orig` argument to be 2018 edition Rust,
/// and will output very readable TypeScript 4, which pollutes global scope.
/// ```
/// # use opinionated_rust_to_typescript::transpile::config::Config;
/// assert_eq!(Config::new().to_string(),
///     "Latest Rust edition (2018), Latest TypeScript (4), Gungho");
/// ```
/// 
/// ### Modifying `Config`
/// Use `rs_edition()`, `strategy()` and `ts_major()` to set the parameters.
/// ```
/// # use opinionated_rust_to_typescript::transpile::config::*;
/// # use opinionated_rust_to_typescript::transpile::rs_to_ts::*;
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
///
pub struct Config {
    /// The edition of Rust that the input code is written in.
    pub rs_edition: RsEdition,
    /// Which strategy to use when transpiling Rust code into TypeScript.
    pub strategy: Strategy,
    /// The major version of TypeScript that `rs_to_ts` should output.
    pub ts_major: TsMajor,
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

