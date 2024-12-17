pub mod include;
use super::types::Macro;

use include::macro_include;

pub static MACRO_LIST: [Macro<'_>; 1] = [Macro {
    symbol: "include",
    expand: macro_include,
    //always_ephemeral: false,
}];
