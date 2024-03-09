use std::env;
use unicode_analyze::Text;

// TODO: Use `clap` to parse arguments, allowing for `--help`, `--license`, `--file`, etc. Blocked
// on binary-only dependencies: see <https://github.com/rust-lang/cargo/issues/1982>.

fn main() {
    let mut args = env::args_os();
    args.next();

    for arg in args {
        let text = Text::parse_os_str(&arg);
        println!("{text}");

        for codepoint in text.codepoints() {
            let (value, character, name) = (
                codepoint.display_value(),
                codepoint.display_character(),
                codepoint.display_name(),
            );

            println!("{value} {character} {name}");
        }
    }
}
