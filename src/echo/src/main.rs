use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Command {
    input: Option<String>,
}

fn main() {
    let args = Command::parse();
    println!(
        "{}",
        if let Some(get_input) = args.input.as_deref() {
            String::from(get_input)
        } else {
            String::new()
        }
    );
}
