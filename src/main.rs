use std::env;
use unicode_analyze::Text;

// TODO Use `clap` to parse arguments, allowing for `--help`, `--license`, `--file`, etc.
// We need to somehow make sure the `clap` dependency is specific to just the binary target.

fn main()
{
    let mut args = env::args_os();
    args.next();

    for arg in args
    {
        let text = Text::parse_os_str(&arg);
        println!("{text}");

        for codepoint in text.codepoints()
        {
            println!(
                "{} {} {}",
                codepoint.display_value(),
                codepoint.display_character(),
                codepoint.display_name(),
            );
        }
    }
}
