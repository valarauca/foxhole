use clap::Arg;

pub fn dump_ast_flag(conflicts: &[&'static str]) -> Arg<'static, 'static> {
    Arg::with_name("dump-ast")
        .long("dump-ast")
        .takes_value(true)
        .multiple(false)
        .next_line_help(true)
        .conflicts_with_all(conflicts)
        .required_unless_all(conflicts)
        .value_name("JSON")
        .help("dump abstract syntax tree in JSON representation")
}
