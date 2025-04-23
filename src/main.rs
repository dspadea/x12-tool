use std::{env, io};
use tabular::{Row, Table};
use x12_types::{
    util::Parser,
    v005010::{Transmission,_276, _277, _834, _835},
};


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    let mut docless = false;

    // Todo: Use clap
    if args.len() > 1 {
        if args[1] == "--docless" {
            docless = true;
        }
    }

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    if docless {
        let segments = raw_parse(&buffer);
        tabular_display(&segments);
        return Ok(());
    }

    // We need to update the x12-types library to parse without knowing the doctype up front.
    let (rest, edi_doc) = Transmission::<_276>::parse(&buffer).expect("Parser error:");

    if rest.trim().len() > 0 {
        eprintln!("WARNING: Input may not be completely parsed. Remaining: {rest}");
    }

    // We need much better TUI here, but at least we can look at the parsed
    // structure more easily than the raw EDI.
    dbg!(edi_doc);

    Ok(())
}

fn raw_parse<'a>(edi: &'a str) -> Vec<Vec<&'a str>> {

    let mut segments = vec![];

    for seg in edi.split("~") {
        if seg.trim().len() == 0 {
            continue;
        }
        let split_seg: Vec<&str> = seg.split("*").collect();
        segments.push(split_seg);
    }

    segments
}

fn tabular_display(segments: &Vec<Vec<&str>>) {

    let cols = segments.iter().map(|s| s.len()).max().unwrap();

    eprintln!("Max segment len = {cols}");

    let table_fmt = (0..=cols).map(|_| "{:<} ").collect::<String>();

    let mut table = Table::new(table_fmt.as_str());
    let mut row = Row::new();
    (0..=cols).for_each(|n| row = row.clone().with_cell(n));
    table.add_row(row);

    for seg in segments {
        let mut row = Row::new();
        seg.iter().for_each(|f| row = row.clone().with_cell(*f));

        for blank_col in 0..=cols-seg.len() {
            row = row.clone().with_cell("");
        }

        table.add_row(row);

    }

    println!("{table}");
}