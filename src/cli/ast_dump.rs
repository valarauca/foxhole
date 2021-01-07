use std::fs::{read_to_string, write};

use crate::{
    cli::{validate_input_file, SubCommand},
    internals::{
        errors::term_errors::HumanReadable,
        parser::generated::{parse_code, serialize_ast},
    },
};
use clap::{App, Arg, ArgMatches};

#[derive(Default)]
pub struct AstDump;

impl SubCommand for AstDump {
    fn build(&self) -> App<'static, 'static> {
        App::new(self.name())
            .version("0.0.1")
            .about("dumps the abstract syntax tree in JSON format")
            .set_term_width(80)
            .arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .index(1)
                    .takes_value(true)
                    .env("FOXHOLE_INPUT_FILE")
                    .value_name("FOXHOLE_OUTPUT_FILE")
                    .next_line_help(true)
                    .help("input source file")
                    .required(true)
                    .validator(validate_input_file),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .env("FOXHOLE_OUTPUT_FILE")
                    .value_name("FOXHOLE_OUTPUT_FILE")
                    .takes_value(true)
                    .required(true)
                    .index(2)
                    .next_line_help(true)
                    .help("output file"),
            )
    }

    fn name(&self) -> &'static str {
        "ast-dump"
    }

    fn exec(&self, args: &ArgMatches<'_>) -> Result<(), String> {
        let input = args.value_of("input").unwrap();

        // load code, parse, and serialize
        let data = match read_to_string(&input) {
            Ok(x) => x,
            Err(e) => {
                return Err(format!(
                    "failed to read input:'{:?}' error:'{:?}'",
                    &input, e
                ))
            }
        };
        let ast = match parse_code::<HumanReadable>(&data) {
            Ok(x) => x,
            Err(e) => {
                let mut s = String::new();
                for err in e {
                    s.push_str(&format!("{}\n", err));
                }
                return Err(s);
            }
        };
        let json = serialize_ast(&ast)?;
        let output = args.value_of("output").unwrap();
        match write(&output, &json) {
            Ok(()) => Ok(()),
            Err(e) => Err(format!(
                "failed to write output to file:'{:?}' error:'{:?}'",
                output, e
            )),
        }
    }
}
