pub mod insert;
pub mod simple_blocks;
pub mod simple_macros;
pub mod template;
use crate::macros::simple_macros::macro_reminder;

use super::types::Macro;

use insert::macro_insert;
use simple_blocks::{macro_comment, macro_for_each_arg, macro_repeat, macro_section};
use simple_macros::{macro_filename, macro_filename_canonical, macro_time};
use template::macro_template;

pub static MACRO_LIST: &'static [Macro] = &[
    // Unscoped
    Macro {
        symbol: "insert", // Inserts another file
        expansion: macro_insert,
        has_scope: false,
        min_args: 1,
        max_args: 1,
    },
    Macro {
        symbol: "time",
        expansion: macro_time,
        has_scope: false,
        min_args: 1,
        max_args: 1,
    },
    Macro {
        symbol: "filename",
        expansion: macro_filename,
        has_scope: false,
        min_args: 0,
        max_args: 0,
    },
    Macro {
        symbol: "filename_canonical",
        expansion: macro_filename_canonical,
        has_scope: false,
        min_args: 0,
        max_args: 0,
    },
    Macro {
        symbol: "reminder",
        expansion: macro_reminder,
        has_scope: false,
        min_args: 1,
        max_args: 1,
    },
    // Scoped
    Macro {
        symbol: "comment", // Nothing
        expansion: macro_comment,
        has_scope: true,
        min_args: 0,
        max_args: 0,
    },
    Macro {
        symbol: "repeat", // Outputs what its give x number of times
        expansion: macro_repeat,
        has_scope: true,
        min_args: 1,
        max_args: 1,
    },
    Macro {
        symbol: "section",
        expansion: macro_section,
        has_scope: true,
        min_args: 0,
        max_args: 1,
    },
    Macro {
        symbol: "template",
        expansion: macro_template,
        has_scope: true,
        min_args: 1,
        max_args: usize::max_value(),
    },
    Macro {
        symbol: "for_each_arg",
        expansion: macro_for_each_arg,
        has_scope: true,
        min_args: 1,
        max_args: usize::max_value(),
    },
];
