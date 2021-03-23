//! Used for returning the result of transpilation.

use super::transpile_error::*;

/// Used for returning the result of transpilation.
/// 
/// When Rust is transpiled to TypeScript, the main program logic is returned
/// inside `main_lines`. But to run it, TypeScript will need some extra code:
/// - `main_section_begins/ends` which wraps `main_lines`
/// - `polyfill_section_begins/ends` which wraps `polyfill_lines`
/// - `type_lines` which declares any enums, interfaces, and other types
pub struct TranspileResult {
    /// If there are no transpilation errors, this vector will be empty.
    pub errors: Vec<TranspileError>,
    /// Lines of TypeScript code
    pub main_lines: Vec<&'static str>,
    /// Should be added before `main`, typically `;r$t$();`
    pub main_section_begins: &'static str,
    /// Should be added after `main`
    pub main_section_ends: &'static str,
    /// For example, `String.prototype.len=function(){return this.length}`
    pub polyfill_lines: Vec<&'static str>,
    /// Typically `;function r$t$(){...};`
    pub polyfill_section_begins: &'static str,
    /// Typically `};`
    pub polyfill_section_ends: &'static str,
    /// For example, `interface String { len(): Number }`
    pub type_lines: Vec<&'static str>,
}

impl TranspileResult {
    /// Creates an empty [`TranspileResult`] object.
    pub fn new() -> Self {
        TranspileResult {
            errors: vec![],
            type_lines: vec![],
            main_lines: vec![],
            main_section_begins: "",
            main_section_ends: "",
            polyfill_lines: vec![],
            polyfill_section_begins: "",
            polyfill_section_ends: "",
        }
    }

    /// Adds a [`ConfigNotImplemented`](
    /// super::transpile_error::TranspileErrorKind) [`TranspileError`](
    /// super::transpile_error::TranspileError) to the `errors` vector.
    pub fn push_config_not_implemented_error(
        mut self,
        column: usize,
        line_number: usize,
        message: &'static str,
    ) -> Self {
        self.errors.push(TranspileError {
            column,
            kind: TranspileErrorKind::ConfigNotImplemented,
            line_number,
            message,
        });
        return self;
    }

    /// Adds a line to the `main_lines` vector.
    pub fn push_main_line(
        mut self,
        line: &'static str,
    ) -> Self {
        self.main_lines.push(line);
        return self;
    }

    /// Concatenates `TranspileResult` to run as standalone TypeScript.
    pub fn to_string(&self) -> String {
        let mut out: String = "".into();

        // Add the main section.
        out.push_str(&self.main_section_begins.to_string());
        for main_line in &self.main_lines {
            out.push_str(&main_line.to_string());
        }
        out.push_str(&self.main_section_ends.to_string());

        // Add the types.
        for type_line in &self.type_lines {
            out.push_str(&type_line.to_string());
        }

        // Add the polyfill section.
        out.push_str(&self.polyfill_section_begins.to_string());
        for polyfill_line in &self.polyfill_lines {
            out.push_str(&polyfill_line.to_string());
        }
        out.push_str(&self.polyfill_section_ends.to_string());

        return out;
    }
}
