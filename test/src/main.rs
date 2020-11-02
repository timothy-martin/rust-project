use lrlex::{LexerBuilder, LexerKind};
use std::collections::HashMap;

fn main() {

    let lex_rule_ids_map: HashMap<String, u32> = HashMap::new();

    LexerBuilder::new()
        .lexerkind(LexerKind::Flex)
        .rule_ids_map(lex_rule_ids_map)
        .process_file_in_src("calc.l");

}
