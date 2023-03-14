use clap::Parser;
use rand::{distributions::*, rngs::StdRng};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    /// cantidad de replicaciones 'n' a realizar
    #[arg(short, long, default_value = "-1")]
    repeticiones: i32,

    /// generar tabla de resultados
    #[arg(short, long)]
    tabla: bool,
}

fn main() {
    let args = Args::parse();
    if args.tabla {
        println!(
            "<table><thead><tr>
                 <td>Repeticiones</td>
                 <td>Estimacion</td>
                 <td>Desviacion Estandar</td>
                 <td>Tiempo</td>
             </tr></thead><tbody>"
        );
    }

    if args.repeticiones == -1 {
        mmc(10, args.tabla);
        mmc(100, args.tabla);
        mmc(1000, args.tabla);
        mmc(10000, args.tabla);
        mmc(100000, args.tabla);
        mmc(1000000, args.tabla);
        mmc(10000000, args.tabla);
        mmc(100000000, args.tabla);
        mmc(1000000000, args.tabla);
    } else {
        mmc(args.repeticiones, args.tabla);
    }

    if args.tabla {
        println!("</tbody></table>");
    }
}

fn mmc(repeticiones: i32, tabla: bool) {
    let start = Instant::now();

    // Aplico metodo de montecarlo
    let mut x = 0.0;
    let mut v = 0.0;

    // Creo funciones distribucion
    let t1_distribtion = Uniform::new_inclusive(40.0, 56.0);
    let t2_distribtion = Uniform::new_inclusive(24.0, 32.0);
    let t3_distribtion = Uniform::new_inclusive(20.0, 40.0);
    let t4_distribtion = Uniform::new_inclusive(16.0, 48.0);
    let t5_distribtion = Uniform::new_inclusive(10.0, 30.0);
    let t6_distribtion = Uniform::new_inclusive(15.0, 30.0);
    let t7_distribtion = Uniform::new_inclusive(20.0, 25.0);
    let t8_distribtion = Uniform::new_inclusive(30.0, 50.0);
    let t9_distribtion = Uniform::new_inclusive(40.0, 60.0);
    let t10_distribtion = Uniform::new_inclusive(8.0, 16.0);

    //let mut rng = rand::thread_rng();
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(24242);

    for _ in 0..repeticiones {
        // Genero valores aleatorios
        let t1 = t1_distribtion.sample(&mut rng);
        let t2 = t2_distribtion.sample(&mut rng);
        let t3 = t3_distribtion.sample(&mut rng);
        let t4 = t4_distribtion.sample(&mut rng);
        let t5 = t5_distribtion.sample(&mut rng);
        let t6 = t6_distribtion.sample(&mut rng);
        let t7 = t7_distribtion.sample(&mut rng);
        let t8 = t8_distribtion.sample(&mut rng);
        let t9 = t9_distribtion.sample(&mut rng);
        let t10 = t10_distribtion.sample(&mut rng);

        // Calculo tiempos totales por tarea
        let t2_total = t1 + t2;
        let t3_total = t1 + t3;
        let t4_total = f64::max(t2_total, t3_total) + t4;
        let t5_total = f64::max(t2_total, t3_total) + t5;
        let t6_total = t3_total + t6;
        let t7_total = t3_total + t7;
        let t8_total = get_max_tiempo(&vec![t4_total, t5_total, t6_total, t7_total]) + t8;
        let t9_total = t5_total + t9;

        // Calculo tiempo total
        let tiempo_total = get_max_tiempo(&vec![t8_total, t9_total]) + t10;

        x += tiempo_total;
        v += tiempo_total.powi(2);
    }

    let f_rep = repeticiones as f64;
    let x = x / f_rep;
    let v = (v / (f_rep * (f_rep - 1.0)) - (x.powi(2) / (f_rep - 1.0))).sqrt();

    let duration = start.elapsed();

    if tabla {
        println!(
            "<tr>
                 <td>{}</td>
                 <td>{}</td>
                 <td>{}</td>
                 <td>{:?}</td>
             </tr>",
            repeticiones, x, v, duration
        );
        return;
    }

    println!(
        "Reps: {}, Estimacion: {}, Desviacion Estandar: {}, Tiempo: {:?}",
        repeticiones, x, v, duration
    );
}

fn get_max_tiempo<'a>(tiempos: &'a Vec<f64>) -> &'a f64 {
    tiempos.iter().max_by(|a, b| a.total_cmp(b)).unwrap()
}
