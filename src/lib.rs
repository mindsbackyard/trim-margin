/* Copyright 2018 Christopher Bacher
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! This crate is intended to ease the use of multi-line strings in Rust.
//! When embedding strings with multiple lines in Rust all whitespaces, tabs, etc. are preserved even if they are just used for layouting one's code nicely.
//!
//! ```
//! fn main() {
//!     println!("-----------------------");
//!     let misrepresented_multiline_string = "
//!         This is string
//!         spans over multiple lines,
//!         but its rendering preserves all whitespaces.
//!
//!         Which is not what we usually intend in this case.
//!     ";
//!     println!("{}", misrepresented_multiline_string);
//!     println!("-----------------------");
//!
//!     println!("-----------------------");
//!     let correctly_layouted_string = "For displaying
//! the a multiline strin properly
//! it would need to be layouted
//! like this.
//!
//! Which is not very nice.";
//!     println!("{}", correctly_layouted_string);
//!     println!("-----------------------");
//! }
//! ```
//!
//! The `trim-margin` crate supports you with the proper layouting.
//! By introducing a margin in the multi-line string the `trim_margin` method can filter out unwanted whitespaces and blank lines.
//!
//! ```
//! extern crate trim_margin;
//! use trim_margin::MarginTrimmable;
//!
//! fn main() {
//!     let multiline_string_with_margin = "
//!         |This string has a margin
//!         |indicated by the '|' character.
//!         |
//!         |The following method call will remove ...
//!         | * a blank first/last line
//!         | * blanks before the margin prefix
//!         | * the margin prefix itself
//!     ".trim_margin().unwrap();
//!     println!("{}", multiline_string_with_margin);
//! }
//! ```

#[cfg(test)] #[macro_use] extern crate galvanic_assert;


/// An interface for removing the margin of multi-line string-like objects.
pub trait MarginTrimmable {
    /// Removes blanks and the `margin_prefix` from multiline strings.
    ///
    /// If the first or last line is blank (contains only whitespace, tabs, etc.) they are removed.
    /// From each remaining line leading blank characters and the subsequent are removed
    ///
    /// # Returns
    /// * The trimmed string or `None` if not every line starts with a `margin_prefix`.
    /// * Strings without line break unmodified
    fn trim_margin_with<M: AsRef<str>>(&self, margin_prefix: M) -> Option<String>;

    /// Short-hand for `trin_margin_with("|")`.
    fn trim_margin(&self) -> Option<String> { self.trim_margin_with("|") }
}

impl<S: AsRef<str>> MarginTrimmable for S {
    fn trim_margin_with<M: AsRef<str>>(&self, margin_prefix: M) -> Option<String> {
        let lines: Vec<_> = self.as_ref().split('\n').map(|line| line.trim_left()).collect();
        if lines.len() <= 1 {
            return Some(self.as_ref().into());
        }

        let mut with_margin: Vec<&str> = Vec::with_capacity(lines.len());
        let mut line_iter = lines.into_iter().peekable();
        if line_iter.peek().map_or(false, |l| l.is_empty()) {
            line_iter.next();
        }

        let prefix = margin_prefix.as_ref();
        while let Some(line) = line_iter.next() {
            let is_last_line = line_iter.peek().is_none();
            if is_last_line && line.is_empty() {
                continue;
            }
            if !line.starts_with(prefix) {
                return None;
            }
            with_margin.push(&line[prefix.len()..]);
        };

        Some(with_margin.join("\n"))
    }
}


#[cfg(test)]
mod tests {
    use galvanic_assert::matchers::*;
    use galvanic_assert::matchers::variant::*;
    use super::*;

    #[test]
    fn should_not_modify_empty_string() {
        assert_that!(&"".trim_margin(), maybe_some(eq(String::new())));
    }

    #[test]
    fn should_not_modify_single_line_string() {
        assert_that!(&"hello, world".trim_margin(), maybe_some(eq("hello, world".into())));
    }

    #[test]
    fn should_trim_margin_of_multiline_string() {
        let txt = "|this
                   |  is a
                   |  multiline string
                   |with margin";
        assert_that!(&txt.trim_margin(),
                     maybe_some(eq(vec!["this", "  is a", "  multiline string", "with margin"].join("\n"))));
    }

    #[test]
    fn should_remove_first_and_last_line_if_blank() {
        let txt = "
            |ignore blank
            |surrounding lines
        ";
        assert_that!(&txt.trim_margin(),
                     maybe_some(eq(vec!["ignore blank", "surrounding lines"].join("\n"))));
    }

    #[test]
    fn should_allow_arbitrary_margin_character() {
        let txt = "
            #ignore blank
            #surrounding lines
        ";
        assert_that!(&txt.trim_margin_with("#"),
                     maybe_some(eq(vec!["ignore blank", "surrounding lines"].join("\n"))));
    }
}
