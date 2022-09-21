use clap::{App, Arg};

fn main() {
    let matches = App::new("wieder")
        .version("0.1.0")
        .author("Paul Vidal <u1d4lp@gmail.com>")
        .about("A simple echo clone")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        /* .arg(
            Arg::with_name("repeat")
                .short("r")
                .long("repeat")
                .value_name("REPEAT")
                .help("Repeat the input text")
                .takes_value(true),
        ) */
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .long("omit-newline")
                .help("Do not print the trailing newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();

    let omit_newline = matches.is_present("omit_newline");

    println!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
