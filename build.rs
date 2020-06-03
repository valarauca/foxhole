use std::error::Error;

use cfgrammar::yacc::YaccKind;
use lrlex::LexerBuilder;
use lrpar::{CTParserBuilder, RecoveryKind};

fn main() -> Result<(), Box<dyn Error>> {
    let master_lex_rules_id_map = CTParserBuilder::new()
        .yacckind(YaccKind::Grmtools)
        .recoverer(RecoveryKind::None)
        .error_on_conflicts(true)
        .module_public(true)
        .process_file(
            "src/internals/parser/generated/parser.y",
            "src/internals/parser/generated/parser.rs",
        )?;

    LexerBuilder::new()
        .rule_ids_map(master_lex_rules_id_map)
        .module_public(true)
        .process_file(
            "src/internals/parser/generated/lexer.l",
            "src/internals/parser/generated/lexer.rs",
        )?;

    Ok(())
}
