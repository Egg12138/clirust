use clap::{App, Arg};


fn main() {

    let matches = App::new("echor") //program name
            .version("0.1.0")
            .author("aydenegg  xoltraman@outlook.com")
            .about("Rust echo simulation")
            .arg(
                Arg::new("liberal")
                    .value_name("LIBERAL")
                    .help("Inpupt String")
                    .required(true)
                    .min_values(1),
                    )
            .arg(
               Arg::new("omit_newline")
                    .short('n')
                    .help("Do not append a newline into the output stream")
                    .takes_value(false),

                )
            .get_matches();//builds a ArgMatch structure 

            let liberal = matches.values_of_lossy("liberal").unwrap();
            let omit_newline = matches.is_present("omit_newline");
            print!("{}{}", liberal.join(" "), if omit_newline { " " } else { "\n" });

}