use minigrep::{run, Config};
use std::{env, process};

fn main() {
    // Here we are using the Config struct from the library crate
    // to parse the arguments and create a Config instance,
    // if the arguments are not enough, we will print an error message
    // and exit the program.
    // unwrap_or_else is a method that takes a closure and returns the value,
    // if it is Ok it returns the value, if it is Err it calls the closure
    // |err| is a closure that takes an argument err and returns a string
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    // If build doesn't implements the Iterator trait on the parameter we need to collect the arguments first

    // If the run function returns an error, we will print the error message and exit the program.
    // Here we don't need to use unwrap_or_else because the run function returns ()
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
