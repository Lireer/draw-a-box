#![no_std]
#![deny(missing_docs)]

//! # Draw a box
//!
//! This crate contains constants for all 128 [unicode box-drawing characters](https://en.wikipedia.org/wiki/Box_Drawing_(Unicode_block))
//! and makes it possible to find the one you need dynamically.
//!
//! ## Constants
//!
//! The constants are separated into modules:
//!
//! - `arc`
//!     - `╭`, `╮`, `╯`, `╰`
//! - `dashed`
//!     - `╌`, `╍`, `╎`, `╏`, `┄`, `┅`, `┆`, `┇`, `┈`, `┉`, `┊`, `┋`
//! - `diagonal`
//!     - `╱`, `╲`, `╳`
//! - `double`
//!     - `═`, `║`, `╒`, `╓`, `╔`, `╕`, `╖`, `╗`, `╘`, `╙`, `╚`, `╛`, `╜`, `╝`, `╞`, `╟`, `╠`, `╡`, `╢`, `╣`, `╤`, `╥`, `╦`, `╧`, `╨`, `╩`, `╪`, `╫`, `╬`
//! - `heavy`
//!     - `━`, `┃`, `┏`, `┓`, `┗`, `┛`, `┣`, `┫`, `┳`, `┻`, `╋`, `╸`, `╹`, `╺`, `╻`
//! - `light`
//!     - `─`, `│`, `┌`, `┐`, `└`, `┘`, `├`, `┤`, `┬`, `┴`, `┼`, `╴`, `╵`, `╶`, `╷`
//! - `light_and_heavy`
//!     - `┍`, `┎`, `┑`, `┒`, `┕`, `┖`, `┙`, `┚`, `┝`, `┞`, `┟`, `┠`, `┡`, `┢`, `┥`, `┦`, `┧`, `┨`, `┩`, `┪`, `┭`, `┮`, `┯`, `┰`, `┱`, `┲`, `┵`, `┶`, `┷`, `┸`, `┹`, `┺`, `┽`, `┾`, `┿`, `╀`, `╁`, `╂`, `╃`, `╄`, `╅`, `╆`, `╇`, `╈`, `╉`, `╊`, `╼`, `╽`, `╾`, `╿`
//!
//! ## Dynamically choosing a character
//!
//! When drawing something thats more than just a simple box, having a way to get a character depending on its properties.
//! Any character that's only made up of light or heavy lines can be found using the `find_character` function.
//! The function takes a `Weight` for each direction and returns the character fitting these weights.
//! Given the weights `Empty` (up), `Empty` (right), `Light` (down), and `Light` (left) the character `┐` would be chosen.
//!
//! ```rust
//! use draw_a_box::{find_character, Weight};
//!
//! let character = find_character(Weight::Light, Weight::Heavy, Weight::Light, Weight::Heavy);
//! assert_eq!(character, "┿");
//! ```

pub mod arc;
pub mod dashed;
pub mod diagonal;
pub mod double;
pub mod heavy;
pub mod light;
pub mod light_and_heavy;

/// The weight of a line.
///
/// The variant names `Light` and `Heavy` are adopted from the unicode character names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Weight {
    /// Signals that a line should not be drawn or is not part of the character, e.g. `" "`.
    Empty,
    /// A line that's not bold, e.g. `"┼"`.
    Light,
    /// A bold line, e.g. `"╋"`.
    Heavy,
}

/// Get a character made up of light and heavy lines by defining the weights of all its lines.
///
/// Any character from [`light`](light), [`heavy`](heavy) and [`light_and_heavy`](light_and_heavy)
/// can be defined this way. Returns " " (space) if all weights are `Empty`.
pub fn find_character(up: Weight, right: Weight, down: Weight, left: Weight) -> &'static str {
    match (up, right, down, left) {
        (Weight::Empty, Weight::Empty, Weight::Empty, Weight::Empty) => " ",
        (Weight::Empty, Weight::Empty, Weight::Empty, Weight::Light) => "╴",
        (Weight::Empty, Weight::Empty, Weight::Empty, Weight::Heavy) => "╸",
        (Weight::Empty, Weight::Empty, Weight::Light, Weight::Empty) => "╷",
        (Weight::Empty, Weight::Empty, Weight::Light, Weight::Light) => "┐",
        (Weight::Empty, Weight::Empty, Weight::Light, Weight::Heavy) => "┑",
        (Weight::Empty, Weight::Empty, Weight::Heavy, Weight::Empty) => "╻",
        (Weight::Empty, Weight::Empty, Weight::Heavy, Weight::Light) => "┒",
        (Weight::Empty, Weight::Empty, Weight::Heavy, Weight::Heavy) => "┓",
        (Weight::Empty, Weight::Light, Weight::Empty, Weight::Empty) => "╶",
        (Weight::Empty, Weight::Light, Weight::Empty, Weight::Light) => "─",
        (Weight::Empty, Weight::Light, Weight::Empty, Weight::Heavy) => "╾",
        (Weight::Empty, Weight::Light, Weight::Light, Weight::Empty) => "┌",
        (Weight::Empty, Weight::Light, Weight::Light, Weight::Light) => "┬",
        (Weight::Empty, Weight::Light, Weight::Light, Weight::Heavy) => "┭",
        (Weight::Empty, Weight::Light, Weight::Heavy, Weight::Empty) => "┎",
        (Weight::Empty, Weight::Light, Weight::Heavy, Weight::Light) => "┰",
        (Weight::Empty, Weight::Light, Weight::Heavy, Weight::Heavy) => "┱",
        (Weight::Empty, Weight::Heavy, Weight::Empty, Weight::Empty) => "╺",
        (Weight::Empty, Weight::Heavy, Weight::Empty, Weight::Light) => "╼",
        (Weight::Empty, Weight::Heavy, Weight::Empty, Weight::Heavy) => "━",
        (Weight::Empty, Weight::Heavy, Weight::Light, Weight::Empty) => "┍",
        (Weight::Empty, Weight::Heavy, Weight::Light, Weight::Light) => "┮",
        (Weight::Empty, Weight::Heavy, Weight::Light, Weight::Heavy) => "┯",
        (Weight::Empty, Weight::Heavy, Weight::Heavy, Weight::Empty) => "┏",
        (Weight::Empty, Weight::Heavy, Weight::Heavy, Weight::Light) => "┲",
        (Weight::Empty, Weight::Heavy, Weight::Heavy, Weight::Heavy) => "┳",
        (Weight::Light, Weight::Empty, Weight::Empty, Weight::Empty) => "╵",
        (Weight::Light, Weight::Empty, Weight::Empty, Weight::Light) => "┘",
        (Weight::Light, Weight::Empty, Weight::Empty, Weight::Heavy) => "┙",
        (Weight::Light, Weight::Empty, Weight::Light, Weight::Empty) => "│",
        (Weight::Light, Weight::Empty, Weight::Light, Weight::Light) => "┤",
        (Weight::Light, Weight::Empty, Weight::Light, Weight::Heavy) => "┥",
        (Weight::Light, Weight::Empty, Weight::Heavy, Weight::Empty) => "╽",
        (Weight::Light, Weight::Empty, Weight::Heavy, Weight::Light) => "┧",
        (Weight::Light, Weight::Empty, Weight::Heavy, Weight::Heavy) => "┪",
        (Weight::Light, Weight::Light, Weight::Empty, Weight::Empty) => "└",
        (Weight::Light, Weight::Light, Weight::Empty, Weight::Light) => "┴",
        (Weight::Light, Weight::Light, Weight::Empty, Weight::Heavy) => "┵",
        (Weight::Light, Weight::Light, Weight::Light, Weight::Empty) => "├",
        (Weight::Light, Weight::Light, Weight::Light, Weight::Light) => "┼",
        (Weight::Light, Weight::Light, Weight::Light, Weight::Heavy) => "┽",
        (Weight::Light, Weight::Light, Weight::Heavy, Weight::Empty) => "┟",
        (Weight::Light, Weight::Light, Weight::Heavy, Weight::Light) => "╁",
        (Weight::Light, Weight::Light, Weight::Heavy, Weight::Heavy) => "╅",
        (Weight::Light, Weight::Heavy, Weight::Empty, Weight::Empty) => "┕",
        (Weight::Light, Weight::Heavy, Weight::Empty, Weight::Light) => "┶",
        (Weight::Light, Weight::Heavy, Weight::Empty, Weight::Heavy) => "┷",
        (Weight::Light, Weight::Heavy, Weight::Light, Weight::Empty) => "┝",
        (Weight::Light, Weight::Heavy, Weight::Light, Weight::Light) => "┾",
        (Weight::Light, Weight::Heavy, Weight::Light, Weight::Heavy) => "┿",
        (Weight::Light, Weight::Heavy, Weight::Heavy, Weight::Empty) => "┢",
        (Weight::Light, Weight::Heavy, Weight::Heavy, Weight::Light) => "╆",
        (Weight::Light, Weight::Heavy, Weight::Heavy, Weight::Heavy) => "╈",
        (Weight::Heavy, Weight::Empty, Weight::Empty, Weight::Empty) => "╹",
        (Weight::Heavy, Weight::Empty, Weight::Empty, Weight::Light) => "┚",
        (Weight::Heavy, Weight::Empty, Weight::Empty, Weight::Heavy) => "┛",
        (Weight::Heavy, Weight::Empty, Weight::Light, Weight::Empty) => "╿",
        (Weight::Heavy, Weight::Empty, Weight::Light, Weight::Light) => "┦",
        (Weight::Heavy, Weight::Empty, Weight::Light, Weight::Heavy) => "┩",
        (Weight::Heavy, Weight::Empty, Weight::Heavy, Weight::Empty) => "┃",
        (Weight::Heavy, Weight::Empty, Weight::Heavy, Weight::Light) => "┨",
        (Weight::Heavy, Weight::Empty, Weight::Heavy, Weight::Heavy) => "┫",
        (Weight::Heavy, Weight::Light, Weight::Empty, Weight::Empty) => "┖",
        (Weight::Heavy, Weight::Light, Weight::Empty, Weight::Light) => "┸",
        (Weight::Heavy, Weight::Light, Weight::Empty, Weight::Heavy) => "┹",
        (Weight::Heavy, Weight::Light, Weight::Light, Weight::Empty) => "┞",
        (Weight::Heavy, Weight::Light, Weight::Light, Weight::Light) => "╀",
        (Weight::Heavy, Weight::Light, Weight::Light, Weight::Heavy) => "╃",
        (Weight::Heavy, Weight::Light, Weight::Heavy, Weight::Empty) => "┠",
        (Weight::Heavy, Weight::Light, Weight::Heavy, Weight::Light) => "╂",
        (Weight::Heavy, Weight::Light, Weight::Heavy, Weight::Heavy) => "╉",
        (Weight::Heavy, Weight::Heavy, Weight::Empty, Weight::Empty) => "┗",
        (Weight::Heavy, Weight::Heavy, Weight::Empty, Weight::Light) => "┺",
        (Weight::Heavy, Weight::Heavy, Weight::Empty, Weight::Heavy) => "┻",
        (Weight::Heavy, Weight::Heavy, Weight::Light, Weight::Empty) => "┡",
        (Weight::Heavy, Weight::Heavy, Weight::Light, Weight::Light) => "╄",
        (Weight::Heavy, Weight::Heavy, Weight::Light, Weight::Heavy) => "╇",
        (Weight::Heavy, Weight::Heavy, Weight::Heavy, Weight::Empty) => "┣",
        (Weight::Heavy, Weight::Heavy, Weight::Heavy, Weight::Light) => "╊",
        (Weight::Heavy, Weight::Heavy, Weight::Heavy, Weight::Heavy) => "╋",
    }
}

#[cfg(test)]
mod tests {
    use crate::{find_character, Weight};

    #[test]
    #[rustfmt::skip]
    fn mapping() {
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Empty, Weight::Empty), " ");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Empty, Weight::Light), "╴");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Empty, Weight::Heavy), "╸");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Light, Weight::Empty), "╷");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Light, Weight::Light), "┐");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Light, Weight::Heavy), "┑");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Heavy, Weight::Empty), "╻");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Heavy, Weight::Light), "┒");
        assert_eq!(find_character(Weight::Empty, Weight::Empty, Weight::Heavy, Weight::Heavy), "┓");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Empty, Weight::Empty), "╶");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Empty, Weight::Light), "─");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Empty, Weight::Heavy), "╾");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Light, Weight::Empty), "┌");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Light, Weight::Light), "┬");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Light, Weight::Heavy), "┭");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Heavy, Weight::Empty), "┎");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Heavy, Weight::Light), "┰");
        assert_eq!(find_character(Weight::Empty, Weight::Light, Weight::Heavy, Weight::Heavy), "┱");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Empty, Weight::Empty), "╺");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Empty, Weight::Light), "╼");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Empty, Weight::Heavy), "━");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Light, Weight::Empty), "┍");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Light, Weight::Light), "┮");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Light, Weight::Heavy), "┯");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Heavy, Weight::Empty), "┏");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Heavy, Weight::Light), "┲");
        assert_eq!(find_character(Weight::Empty, Weight::Heavy, Weight::Heavy, Weight::Heavy), "┳");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Empty, Weight::Empty), "╵");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Empty, Weight::Light), "┘");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Empty, Weight::Heavy), "┙");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Light, Weight::Empty), "│");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Light, Weight::Light), "┤");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Light, Weight::Heavy), "┥");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Heavy, Weight::Empty), "╽");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Heavy, Weight::Light), "┧");
        assert_eq!(find_character(Weight::Light, Weight::Empty, Weight::Heavy, Weight::Heavy), "┪");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Empty, Weight::Empty), "└");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Empty, Weight::Light), "┴");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Empty, Weight::Heavy), "┵");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Light, Weight::Empty), "├");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Light, Weight::Light), "┼");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Light, Weight::Heavy), "┽");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Heavy, Weight::Empty), "┟");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Heavy, Weight::Light), "╁");
        assert_eq!(find_character(Weight::Light, Weight::Light, Weight::Heavy, Weight::Heavy), "╅");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Empty, Weight::Empty), "┕");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Empty, Weight::Light), "┶");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Empty, Weight::Heavy), "┷");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Light, Weight::Empty), "┝");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Light, Weight::Light), "┾");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Light, Weight::Heavy), "┿");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Heavy, Weight::Empty), "┢");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Heavy, Weight::Light), "╆");
        assert_eq!(find_character(Weight::Light, Weight::Heavy, Weight::Heavy, Weight::Heavy), "╈");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Empty, Weight::Empty), "╹");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Empty, Weight::Light), "┚");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Empty, Weight::Heavy), "┛");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Light, Weight::Empty), "╿");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Light, Weight::Light), "┦");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Light, Weight::Heavy), "┩");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Heavy, Weight::Empty), "┃");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Heavy, Weight::Light), "┨");
        assert_eq!(find_character(Weight::Heavy, Weight::Empty, Weight::Heavy, Weight::Heavy), "┫");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Empty, Weight::Empty), "┖");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Empty, Weight::Light), "┸");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Empty, Weight::Heavy), "┹");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Light, Weight::Empty), "┞");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Light, Weight::Light), "╀");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Light, Weight::Heavy), "╃");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Heavy, Weight::Empty), "┠");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Heavy, Weight::Light), "╂");
        assert_eq!(find_character(Weight::Heavy, Weight::Light, Weight::Heavy, Weight::Heavy), "╉");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Empty, Weight::Empty), "┗");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Empty, Weight::Light), "┺");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Empty, Weight::Heavy), "┻");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Light, Weight::Empty), "┡");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Light, Weight::Light), "╄");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Light, Weight::Heavy), "╇");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Heavy, Weight::Empty), "┣");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Heavy, Weight::Light), "╊");
        assert_eq!(find_character(Weight::Heavy, Weight::Heavy, Weight::Heavy, Weight::Heavy), "╋");
    }
}
