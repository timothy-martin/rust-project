use lrlex::{LexerBuilder, LexerKind};

// #[derive(Debug)]
// pub enum LexerKind {
//     LRNonStreamingLexer,
//     Lex,
//     Flex,
// }

fn main() {

    let lex_type = LexerKind::Lex;
    println!("{:?}", lex_type);
    let flex_type = LexerKind::Flex;
    println!("{:?}", flex_type);

    LexerBuilder::new()
        .lexerkind(LexerKind::Flex)
        .rule_ids_map(lex_rule_ids_map)
        .process_file_in_src("calc.l")?;

}
