use clap::Parser;
use rand::distributions::*;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    /// cantidad de replicaciones 'n' a realizar
    #[arg(short, long, default_value = "-1")]
    repeticiones: i32,
}

fn main() {
    let args = Args::parse();
    if args.repeticiones == -1 {
        MMC(10);
        MMC(100);
        MMC(1000);
        MMC(10000);
        MMC(100000);
        MMC(1000000);
        MMC(10000000);
        MMC(100000000);
        MMC(1000000000);
    } else {
        MMC(args.repeticiones);
    }
}

fn MMC(repeticiones: i32) {
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

    let mut rng = rand::thread_rng();

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

        let mut t_totales: Vec<f32> = Vec::new();

        // Calculo tiempos totales por tarea
        t_totales.push(t1);
        t_totales.push(t1 + t2);
        t_totales.push(t1 + t3);
        t_totales.push(f32::max(t_totales[1], t_totales[2]) + t4);
        t_totales.push(f32::max(t_totales[1], t_totales[2]) + t5);
        t_totales.push(t3 + t6);
        t_totales.push(t3 + t7);
        t_totales.push(
            get_max_tiempo(&vec![
                t_totales[3],
                t_totales[4],
                t_totales[5],
                t_totales[6],
            ]) + t8,
        );
        t_totales.push(t_totales[4] + t9);
        t_totales.push(get_max_tiempo(&vec![t_totales[6], t_totales[7], t_totales[8]]) + t10);

        // Calculo tiempo total
        let tiempo_total = get_max_tiempo(&t_totales);

        x += tiempo_total;
        v += tiempo_total.powi(2);
    }

    let f_rep = repeticiones as f32;
    let x = x / f_rep;
    let v = v / (f_rep * (f_rep - 1.0)) - (x.powi(2) / (f_rep - 1.0));

    let duration = start.elapsed();

    println!(
        "Reps: {}, Estimacion: {}, Desviacion Estandar: {}, Tiempo: {:?}",
        repeticiones, x, v, duration
    );
}

fn get_max_tiempo<'a>(tiempos: &'a Vec<f32>) -> &'a f32 {
    tiempos.iter().max_by(|a, b| a.total_cmp(b)).unwrap()
}
