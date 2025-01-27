use clap::Parser;
use cli_table::{format, print_stdout, Cell, Table};
use std::{fs, slice::SliceIndex};

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
    if args.quiet {
        println!("{contents}");
    } else {
        let sliced_content = contents.lines();
        let line_number: u16 = 1;
        let each_line = vec![];
        for content in sliced_content {
            let temporary_vector = vec![line_number.cell()];
            temporary_vector.push(sliced_content.next());
            // sliced_content.next();
            line_number += 1;
            each_line.push(temporary_vector);
        }
        let table = vec![
            // vec!["h".cell(), "h".cell()],
            // vec!["stp".cell(), "wwd".cell()],

        ]
        .table()
        .title(vec![
            "File:".cell(),
            file_path.cell().justify(format::Justify::Right),
        ]);
        assert!(print_stdout(table).is_ok());
    }
}
