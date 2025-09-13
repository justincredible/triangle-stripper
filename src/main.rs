use std::collections::VecDeque;

use clap::Parser;

mod stripper;
mod triangle;

/// Determine if a triangle list can be stripped
#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    #[arg(short, long, help = "Print the parsed input")]
    echo_input: bool,
    #[arg(required = true)]
    index_list: Vec<u32>,
}

fn main() -> Result<(), stripper::Failure> {
    let args = Args::parse();

    let (degenerates, triples): (Vec<_>, Vec<_>) = args.index_list
        .chunks_exact(3)
        .partition(|ch| ch[0] == ch[1] || ch[0] == ch[2] || ch[1] == ch[2]);

    degenerates
        .into_iter()
        .fold((), |_, ch| eprintln!("Skipping degenerate input: ({}, {}, {})", ch[0], ch[1], ch[2]));

    let triangle_list: VecDeque<_> = triples
        .into_iter()
        .map(|ch| (*ch).into())
        .collect();

    if args.echo_input {
        if triangle_list.is_empty() {
            println!("[]");
        } else {
            print!("[{}", triangle_list[0]);
            for triangle in triangle_list.iter().skip(1) {
                print!(", {}", triangle);
            }
            println!("]");
        }
    } else {
        let strip = stripper::strip(triangle_list)?;
        println!("{:?}", strip);
    }

    Ok(())
}

