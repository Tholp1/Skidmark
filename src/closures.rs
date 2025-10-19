// Closures are essentally blocked macros that change behavior on symbol instead of name
// Instances of most types of closures can be named as sections ...
// ... to work with !insert() to pick certain parts out of a file

use boa_engine::Context;

use crate::{
    project::ProjectContext,
    types::{SkidContext, Token},
};

type ClosureFunction = fn(&[Token], &mut ProjectContext, &mut SkidContext) -> Vec<Token>;

pub struct Closure {
    pub opener: &'static str,
    pub opener2: &'static str,
    pub closer: &'static str,
    pub function: ClosureFunction,
}

// (opener) name (opener2) ... (closer)

// << js ! .. >>
// <!-- comment -->
// [ name {{ .. }}]

// <!-- ... --> comment
// ?name<< >> named js
// ?name[[ ]] named section
// ?<< >> js
// ?[[ ]] section
// ?name^[[]] named emphemeral section
// ?name-[[]] named inverted section

pub static CLOSURE_LIST: &'static [Closure] = &[
    Closure {
        opener: "?",
        opener2: "<<",
        closer: ">>",
        function: closure_js,
    },
    Closure {
        opener: "<!--",
        opener2: "", // blank means it doesnt accept a name
        closer: "-->",
        function: closure_comment,
    },
    Closure {
        opener: "?",
        opener2: "{{",
        closer: "}}",
        function: closure_section,
    },
];

fn closure_comment(
    _tokens: &[Token],
    _project_context: &mut ProjectContext,
    _skid_context: &mut SkidContext,
) -> Vec<Token> {
    Vec::new()
}

fn closure_section(
    tokens: &[Token],
    _project_context: &mut ProjectContext,
    _skid_context: &mut SkidContext,
) -> Vec<Token> {
    tokens.to_vec()
}

fn closure_js(
    tokens: &[Token],
    project_context: &mut ProjectContext,
    skid_context: &mut SkidContext,
) -> Vec<Token> {
    Vec::new()
}
