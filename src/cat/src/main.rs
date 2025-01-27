use clap::Parser;
use cli_table::{format, print_stdout, Cell, Table};
use std::fs;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
struct Command {
    file: Option<String>,

    #[arg(short, long, action, default_missing_value = "true")]
    quiet: bool,
}

fn main() {
    let args = Command::parse();
    if args.file.is_none() {
        println!("E: File not provided!!!");
        return;
    }
    let file_path = if let Some(get_file) = args.file.as_deref() {
        String::from(get_file)
    } else {
        return;
    };
    let contents = fs::read_to_string(&file_path).expect("E: {args.file} - Perission denied.");
    // let table = vec![].table().title(vec!["File:".cell()]);
    if args.quiet {
        println!("{contents}");
    } else {
        let sliced_content = contents.lines();
        let table = vec![
            vec!["h".cell(), "h".cell()],
            vec!["stp".cell(), "wwd".cell()],
        ]
        .table()
        .title(vec![
            "File:".cell(),
            file_path.cell().justify(format::Justify::Right),
        ]);
        assert!(print_stdout(table).is_ok());
    }
}
