use lrlex::{LexerBuilder, LexerKind};

fn main() {
    LexerBuilder::<u32>::new()
        .lexerkind(LexerKind::Flex)
        .caseless()
        .process_file_in_src("java.l")
        .unwrap();
}