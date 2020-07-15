mod ast;

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
