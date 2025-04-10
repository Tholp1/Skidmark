pub mod clear;
pub mod insert;
pub mod simple_blocks;
use super::types::Macro;

use clear::macro_clear;
use insert::macro_insert;
use simple_blocks::{macro_comment, macro_repeat};

pub static MACRO_LIST: [Macro<'_>; 4] = [
    // Unscoped
    Macro {
        symbol: "insert", // Inserts another file
        expand: macro_insert,
        has_scope: false,
    },
    Macro {
        symbol: "clear", // Clears text buffer
        expand: macro_clear,
        has_scope: false,
    },
    // Scoped
    Macro {
        symbol: "comment", // Nothing
        expand: macro_comment,
        has_scope: true,
    },
    Macro {
        symbol: "repeat", // Outputs what its give x number of times
        expand: macro_repeat,
        has_scope: true,
    },
];
