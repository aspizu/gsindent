/// gsindent takes goboscript source-code from standard input and outputs formatted
/// (or pretty-printed) source-code to the standard output.
use regex::Regex;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};
use std::thread;

fn main() {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    let transforms = [
        // Transform onflag, onclick and onclone to be treated as C function declarations
        // because indent will not cope with goboscript declaration syntax.
        (
            (
                Regex::new(r"(?m)^(?<keyword>onflag|onclick|onclone)\s*\{").unwrap(),
                "$keyword() {",
            ),
            (
                Regex::new(r"(?m)^(?<keyword>onflag|onclick|onclone)()\s*\{").unwrap(),
                "$keyword {",
            ),
        ),
        // Add brackets around the arguments in function declarations.
        (
            (
                Regex::new(r"(?m)^(?<keyword>def|nowarp\s*def)\s*(?<name>[_a-zA-Z0-9]+)\s*(?<args>[^{]*)\{")
                    .unwrap(),
                "$keyword $name($args) {",
            ),
            (
                Regex::new(r"(?m)^(?<keyword>def|nowarp\s*def)\s*(?<name>[_a-zA-Z0-9]+)\s*\((?<args>[^)]*)\)\s*\{")
                    .unwrap(),
                "$keyword $name $args {",
            ),
        ),
    ];
    for ((apply, apply_repl), _) in &transforms {
        buf = apply.replace_all(&buf, *apply_repl).into_owned();
    }
    // TODO: Transform buf by calling __gsindent
    let mut child = Command::new("__gsindent")
        .args(["-kr", "-nut", "-l88", "-nce", "-brf"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    thread::spawn(move || {
        stdin.write_all(buf.as_bytes()).unwrap();
    });
    let output = child.wait_with_output().unwrap();
    let mut buf = String::from_utf8_lossy(output.stdout.as_slice()).into_owned();
    for (_, (deapply, deapply_repl)) in &transforms {
        buf = deapply.replace_all(&buf, *deapply_repl).into_owned();
    }
    io::stdout().write(buf.as_bytes()).unwrap();
}
