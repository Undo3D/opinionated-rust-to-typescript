//! This module contains the library’s main function, `rs_to_ts()`.

use crate::rs2018_ts4::rs2018_ts4_gungho::rs2018_ts4_gungho;

/// The edition of Rust that the input code is written in.
#[derive(PartialEq)]
pub enum RsEdition {
    /// PLACEHOLDER: We have no plans to support 2015 at present.
    Rs2015,
    /// Currently, only the 2018 edition of Rust is supported.
    Rs2018,
    /// The most recent Rust edition that this library supports.
    Latest,
}

/// Which strategy to use when transpiling Rust code into TypeScript.
#[derive(PartialEq)]
pub enum Strategy {
    /// PLACEHOLDER: We have no plans to add the Cautious strategy at present.
    /// 
    /// A strategy which is quite verbose, and looks very different to the input
    /// Rust code, but does not pollute global scope.
    Cautious,
    /// Currently the only supported strategy. Pollutes global scope by adding
    /// methods to native `prototype` objects, eg `String.prototype.len()`.
    /// Looks very similar to the input Rust code, and attempts to preserve line
    /// numbers.
    Gungho,
}

/// The major version of TypeScript that `rs_to_ts` should output.
#[derive(PartialEq)]
pub enum TsMajor {
    /// PLACEHOLDER: We have no plans to support TypeScript 3 at present.
    Ts3,
    /// Currently, only TypeScript 4 is supported.
    Ts4,
    /// The most recent TypeScript major-version that this library supports.
    Latest,
}

/// A configuration object. Controls how the Rust code should be transpiled
/// to TypeScript.
/// 
/// Info about the Builder Pattern:
/// <https://doc.rust-lang.org/1.0.0/style/ownership/builders.html>
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
            RsEdition::Rs2015 => "Rust edition 2015, ",
            RsEdition::Rs2018 => "Rust edition 2018, ",
            RsEdition::Latest => "Latest Rust edition (2018), ",
        }.into());
        out.push_str(match &self.ts_major {
            TsMajor::Ts3 => "TypeScript 3, ",
            TsMajor::Ts4 => "TypeScript 4, ",
            TsMajor::Latest => "Latest TypeScript (4), ",
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
/// * `config` Defines code versions and transpilation strategy
/// 
/// ### Returns
/// @TODO document what this function returns
pub fn rs_to_ts(
    raw: &str,
    config: Config,
) -> &str {
    if config.rs_edition == RsEdition::Rs2015 {
        return "// RsEdition::Rs2015 is not implemented yet";
    }
    if config.strategy == Strategy::Cautious {
        return "// Strategy::Cautious is not implemented yet";
    }
    if config.ts_major == TsMajor::Ts3 {
        return "// TsMajor::Ts3 is not implemented yet";
    }
    return rs2018_ts4_gungho(raw);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_works_using_the_builder_pattern() {
        assert_eq!(Config::new().to_string(),
        "Latest Rust edition (2018), Latest TypeScript (4), Gungho");
        assert_eq!(Config::new().rs_edition(RsEdition::Rs2015).to_string(),
        "Rust edition 2015, Latest TypeScript (4), Gungho");
        assert_eq!(Config::new().strategy(Strategy::Cautious).to_string(),
        "Latest Rust edition (2018), Latest TypeScript (4), Cautious");
        assert_eq!(Config::new().ts_major(TsMajor::Ts3).to_string(),
        "Latest Rust edition (2018), TypeScript 3, Gungho");
        assert_eq!(Config::new()
            .strategy(Strategy::Cautious)
            .rs_edition(RsEdition::Rs2015)
            .ts_major(TsMajor::Ts3)
            .rs_edition(RsEdition::Rs2018)
            .ts_major(TsMajor::Ts4)
            .to_string(),
        "Rust edition 2018, TypeScript 4, Cautious");
    }

    #[test]
    fn rs_to_ts_rejects_placeholder_config() {
        assert_eq!(rs_to_ts("Nope",
            Config::new().rs_edition(RsEdition::Rs2015)
        ), "// RsEdition::Rs2015 is not implemented yet");
        assert_eq!(rs_to_ts("Nope",
            Config::new().strategy(Strategy::Cautious)
        ), "// Strategy::Cautious is not implemented yet");
        assert_eq!(rs_to_ts("Nope",
            Config::new().ts_major(TsMajor::Ts3)
        ), "// TsMajor::Ts3 is not implemented yet");
    }
}
