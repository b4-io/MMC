use clap::{command, Parser};

mod ejercicio1;
mod ejercicio2;

#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    /// cantidad de replicaciones 'n' a realizar
    #[arg(short = 'n', long, default_value = "1000")]
    repeticiones: i32,

    /// ejercicio a ejecutar (1 o 2)
    #[arg(short, long)]
    ejercicio: i8,

    /// eliminar (no) restricciones
    #[arg(long = "nr")]
    no_restricciones: bool,
}

fn main() {
    let args = Args::parse();

    if args.ejercicio == 1 {
        ejercicio1::ejercicio1(args.repeticiones, args.no_restricciones);
    } else {
        ejercicio2::ejercicio2();
    }
}
