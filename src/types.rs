use std::sync::Mutex;

pub struct Token {
    pub contents: String,
    pub origin_file: String,
    pub line_number: u32,
}

pub enum BlockEdgeType {
    FileStart,
    FileEnd,
    Start,
    End,
}

// A 'Block' is what im calling the enclosed scope of a macro
pub struct BlockEdge {
    pub edge_type: BlockEdgeType,
    pub tokens_to_next_edge: u64,
}

pub struct InputFile {
    pub filename_input: String,
    pub filename_skidout: String,
    pub filename_htmlout: String,
    pub tokens: Vec<Token>,
    pub block_edges: Vec<BlockEdge>,
}

type MacroExpansion = fn(&InputFile, &Vec<String>) -> Vec<Token>;
pub struct Macro<'a> {
    pub symbol: &'a str,
    pub expand: MacroExpansion,
    //pub always_ephemeral: bool, // This wont be included from other files
}

impl InputFile {
    pub fn new() -> InputFile {
        InputFile {
            filename_input: "".to_string(),
            filename_skidout: "".to_string(),
            filename_htmlout: "".to_string(),
            tokens: Vec::new(),
            block_edges: Vec::new(),
        }
    }
}

impl Token {
    pub fn new(contents: String, origin_file: String, line_number: u32) -> Token {
        Token {
            contents: contents,
            origin_file: origin_file,
            line_number: line_number,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        return self.contents.clone();
    }
}
