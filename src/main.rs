

use std::{ io};
use clap::Parser as _clap_parser;
use csv::WriterBuilder;
use tabular::{Row, Table};
use x12_types::{
    util::Parser as x12_parser,
    v005010::{Transmission,_276},
};
use log::warn;
use crate::cli::{Cli, DoclessOutputMode};

mod cli;


fn main() -> io::Result<()> {

    pretty_env_logger::init();

    let cli = Cli::parse();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    if cli.docless {
        let mut segments = raw_parse(&buffer);

        // CSV and Tabular might not make sense for "structured doc" mode. Need to think about this.
        match cli.docless_output_mode.unwrap_or(DoclessOutputMode::Tabular) {
            DoclessOutputMode::CSV => {csv_display(&mut segments)}
            DoclessOutputMode::Tabular => {tabular_display(&cli, &segments)}
            DoclessOutputMode::JSON => {json_display(&segments)}
        }
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

fn tabular_display(cli: &Cli, segments: &Vec<Vec<&str>>) {

    let cols = segments.iter().map(|s| s.len()).max().unwrap();

    let table_fmt = (0..cols).map(|_| "{:<} ").collect::<String>();

    let mut table = Table::new(table_fmt.as_str());
    let mut row = Row::new().with_cell("SEG");
    (1..cols).for_each(|n| row = row.clone().with_cell(format!("{n:0>2}")));
    table.add_row(row);

    let mut row = Row::new();
    (0..cols).for_each(|_| row = row.clone().with_cell("-----"));
    table.add_row(row);

    let mut transaction_set_type = None;
    let mut seg_count = 0;

    for seg in segments {

        if cli.tabular_show_txn_sets && seg[0] == "ST" {
            table.add_heading(format!("\n\n----EDI {} Transaction Set ----", seg[1]));
            transaction_set_type = Some(seg[1])
        }

        if seg[0] != "ST" && seg[0] != "SE" {
            seg_count += 1;
        }

        let mut row = Row::new();
        seg.iter().for_each(|f| row = row.clone().with_cell(*f));

        for _blank_col in 0..cols-seg.len() {
            row = row.clone().with_cell("");
        }

        table.add_row(row);

        if cli.tabular_show_txn_sets && seg[0] == "SE" {
            table.add_heading(format!("---- END EDI {} Transaction Set (Declared {} segments, found {}) ----\n\n", transaction_set_type.unwrap(), seg[1], seg_count));
        }

        if (!cli.tabular_show_txn_sets) && seg[0] == "SE" && seg[1].parse::<u16>().expect("Invalid SE01 value") != seg_count {
            warn!("SE declares {} segments, but counted {}", seg[1], seg_count);
        }

    }

    println!("{table}");
}

fn json_display(segments: &Vec<Vec<&str>>) {
    println!("{}", serde_json::to_string_pretty(segments).expect("Error converting to JSON"));
}

fn csv_display(segments: &mut Vec<Vec<&str>>) {

    let cols = segments.iter().map(|s| s.len()).max().unwrap();

    let mut wtr = WriterBuilder::new().from_writer(vec![]);

    let mut headers = vec!["SEG".to_string()];
    (1..cols).for_each(|fld| headers.push(format!("{fld:0>2}")));

    wtr.write_record(headers).expect("Failed to write CSV headers");

    for mut rec in segments {

        let mut row = vec![];
        let pad = cols-rec.len();
        row.append(&mut rec);
        row.append(&mut vec![""].repeat(pad));

        wtr.write_record(row).expect("Failed to write CSV");
    }

    println!("{w}", w=String::from_utf8(wtr.into_inner().expect("Failed to expose inner writer")).expect("Failed to read UTF8 bytes"));
}