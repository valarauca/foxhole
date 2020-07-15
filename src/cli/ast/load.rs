use clap::Arg;

pub fn load_ast_flag(conflicts: &[&'static str]) -> Arg<'static, 'static> {
    Arg::with_name("load-ast")
        .long("load-ast")
        .takes_value(true)
        .multiple(false)
        .next_line_help(true)
        .conflicts_with_all(conflicts)
        .required_unless_all(conflicts)
        .value_name("JSON")
        .help("load abstract syntax tree from JSON representation")
        .validator(crate::cli::validate_input_file)
}
