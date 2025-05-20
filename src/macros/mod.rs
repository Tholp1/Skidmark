pub mod insert;
pub mod simple_blocks;
pub mod simple_macros;
pub mod template;
use super::types::Macro;

use insert::macro_insert;
use simple_blocks::{macro_comment, macro_for_each_arg, macro_repeat, macro_section};
use simple_macros::{macro_clear, macro_filename, macro_filename_canonical, macro_time};
use template::macro_template;

pub static MACRO_LIST: &'static [Macro<'_>] = &[
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
    Macro {
        symbol: "time",
        expand: macro_time,
        has_scope: false,
    },
    Macro {
        symbol: "filename",
        expand: macro_filename,
        has_scope: false,
    },
    Macro {
        symbol: "filename_canonical",
        expand: macro_filename_canonical,
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
    Macro {
        symbol: "section",
        expand: macro_section,
        has_scope: true,
    },
    Macro {
        symbol: "template",
        expand: macro_template,
        has_scope: true,
    },
    Macro {
        symbol: "for_each_arg",
        expand: macro_for_each_arg,
        has_scope: true,
    },
];
