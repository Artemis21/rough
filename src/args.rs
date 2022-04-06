use std::path::PathBuf;

const USAGE: &str = "rough
Artemis <me@arty.li>
Render a Rough site.

USAGE:
    rough <SRC> <OUT>

ARGS:
    <SRC>    The path to the folder containing the site source.
    <OUT>    The path to a folder to write the compiled site to.
";

pub fn parse() -> Option<(PathBuf, PathBuf)> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        print!("{}", USAGE);
        return None;
    }
    if args.iter().any(|arg| arg.starts_with('-')) {
        print!("{}", USAGE);
        return None;
    }
    Some(((&args[1]).into(), (&args[2]).into()))
}
