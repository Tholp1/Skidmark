pub mod for_each;
pub mod insert;
pub mod simple_blocks;
pub mod simple_macros;
pub mod template;

use super::types::Macro;
use for_each::*;
use insert::macro_insert;
use simple_blocks::*;
use simple_macros::*;
use template::macro_template;

pub static MACRO_LIST: &'static [Macro] = &[
    // Unscoped
    Macro {
        symbol: "insert", // Inserts another file
        expansion: macro_insert,
        takes_block: false,
        min_args: 1,
        max_args: 1,
    },
    Macro {
        symbol: "time",
        expansion: macro_time,
        takes_block: false,
        min_args: 1,
        max_args: 1,
    },
    Macro {
        symbol: "filename",
        expansion: macro_filename,
        takes_block: false,
        min_args: 0,
        max_args: 0,
    },
    Macro {
        symbol: "filename_canonical",
        expansion: macro_filename_canonical,
        takes_block: false,
        min_args: 0,
        max_args: 0,
    },
    Macro {
        symbol: "reminder",
        expansion: macro_reminder,
        takes_block: false,
        min_args: 1,
        max_args: 1,
    },
    Macro {
        symbol: "output_filename",
        expansion: macro_output_filename,
        takes_block: false,
        min_args: 0,
        max_args: 1,
    },
    // Scoped
    Macro {
        symbol: "comment", // Nothing
        expansion: macro_comment,
        takes_block: true,
        min_args: 0,
        max_args: 0,
    },
    Macro {
        symbol: "repeat", // Outputs what its give x number of times
        expansion: macro_repeat,
        takes_block: true,
        min_args: 1,
        max_args: 1,
    },
    Macro {
        symbol: "section",
        expansion: macro_section,
        takes_block: true,
        min_args: 0,
        max_args: 1,
    },
    Macro {
        symbol: "template",
        expansion: macro_template,
        takes_block: true,
        min_args: 1,
        max_args: usize::max_value(),
    },
    Macro {
        symbol: "for_each_arg",
        expansion: macro_for_each_arg,
        takes_block: true,
        min_args: 1,
        max_args: usize::max_value(),
    },
    Macro {
        symbol: "for_each_file_in_group",
        expansion: macro_for_each_file_in_group,
        takes_block: true,
        min_args: 2,
        max_args: 2,
    },
];
