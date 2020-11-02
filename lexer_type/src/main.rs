use lrlex::{LexerBuilder, LexerKind};

fn main() {

    LexerBuilder::new()
        .lexerkind(LexerKind::Flex)
        .rule_ids_map(lex_rule_ids_map)
        .process_file_in_src("calc.l")?;

}
