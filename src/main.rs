use std::env::args;

mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod utils;

fn main() {
    let arg_values = args().collect::<Vec<_>>();
    let first_arg = arg_values.get(1).expect("Provide the module as arg, e.g. d01");
    match first_arg.as_str() {
        "d01" => d01::main(),
        "d02" => d02::main(),
        "d03" => d03::main(),
        "d04" => d04::main(),
        "d05" => d05::main(),
        "d06" => d06::main(),
        "d07" => d07::main(),
        "d08" => d08::main(),
        "d09" => d09::main(),
        "d10" => d10::main(),
        "d11" => d11::main(),
        _ => println!("Invalid argument: {}", first_arg),
    }
}
