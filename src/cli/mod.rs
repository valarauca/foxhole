use clap::App;

mod ast_dump;
use self::ast_dump::AstDump;

mod traits;
pub use self::traits::SubCommand;

pub fn run() -> Result<(), String> {
    let v: Vec<Box<dyn SubCommand>> = vec![Box::new(AstDump::default())];
    let mut app: App<'static, 'static> = App::new("foxhole");

    for item in v.iter() {
        app = app.subcommand(item.build());
    }

    let args = app.get_matches();
    for item in v.iter() {
        let (name, args) = args.subcommand();
        if name == item.name() {
            let args = match args {
                Option::None => return Err(format!("no arguments passed")),
                Option::Some(args) => args,
            };
            return item.exec(args);
        }
    }
    Err(format!("unrecongized command"))
}

const EOL: &'static str = {
    #[cfg(target_family = "unix")]
    {
        "\n"
    }

    #[cfg(target_family = "windows")]
    {
        "\r\n"
    }
};

/// used in a lot of sub-modules
pub fn validate_input_file(arg: String) -> Result<(), String> {
    use std::borrow::Cow;
    use std::io::ErrorKind;

    let meta = match std::fs::metadata(&arg) {
        Ok(m) => m,
        Err(e) => return Err(format!("error on: {}{}{:?}{}", &arg, EOL, e, EOL)),
    };
    if meta.is_dir() {
        return Err(format!("error on: {}{}is a directory{}", &arg, EOL, EOL));
    }
    if meta.len() == 0 {
        return Err(format!("error on: {}{}length is zero{}", &arg, EOL, EOL));
    }
    match std::fs::read_to_string(&arg) {
        Ok(_) => Ok(()),
        Err(ref e) => {
            let msg = match e.kind() {
                ErrorKind::Interrupted => Cow::from("file contains non-utf8 data"),
                ErrorKind::PermissionDenied => Cow::from("cannot access"),
                k => Cow::from(format!("{:?}", k)),
            };
            Err(format!("error on: {}{}{}{}", &arg, EOL, msg, EOL))
        }
    }
}
