pub mod clear;
pub mod include;
use super::types::Macro;

use clear::macro_clear;
use include::macro_include;

pub static MACRO_LIST: [Macro<'_>; 2] = [
    Macro {
        symbol: "include",
        expand: macro_include,
    },
    Macro {
        symbol: "clear",
        expand: macro_clear,
    },
];
