use clap::Parser;
use rand::{distributions::*, rngs::StdRng};
use statrs::distribution::{ContinuousCDF, Normal};
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

    mmc(args.repeticiones);
}

fn mmc(repeticiones: i32) {
    // Creo funciones distribucion
    let unif_distribution = Uniform::new_inclusive(0.0, 1.0);
    let n = Normal::new(0.0, 1.0).unwrap();

    //let mut rng = rand::thread_rng();
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(24242);

    let mountain_function = |x: f32, y: f32| {
        let h: f32 = 8.0;
        let r: f32 = 0.4;
        let c: (f32, f32) = (0.5, 0.5);

        // check if point is inside the circle
        if (x - c.0).powi(2) + (y - c.1).powi(2) > r.powi(2) {
            0.0
        } else {
            h - (h / r) * ((x - c.0).powi(2) + (y - c.1).powi(2)).sqrt()
        }
    };
    // Aplico metodo de montecarlo
    let mut s = 0.0;
    let mut t = 0.0;

    let start = Instant::now();

    for j in 1..=repeticiones {
        // Genero valores aleatorios
        let x = unif_distribution.sample(&mut rng);
        let y = unif_distribution.sample(&mut rng);

        // Obtengo valor de la funcion en el punto
        let height = mountain_function(x, y);

        // Actualizo los acumuladores
        if j > 1 {
            t += (1.0 - 1.0 / j as f32) * (height - s / (j as f32 - 1.0)).powi(2);
        }
        s += height;
    }

    let f_rep = repeticiones as f32;

    // Estimacion
    let s_lebesgue = s / f_rep;
    // Varianza puntual de la funcion
    let v = t / (f_rep - 1.0);
    // Varianza de la estimacion
    let v_lebesgue = v / f_rep;

    let intevalo = (
        s_lebesgue - n.inverse_cdf(0.975) as f32 * (v / f_rep).sqrt(),
        s_lebesgue + n.inverse_cdf(0.975) as f32 * (v / f_rep).sqrt(),
    );

    let duration = start.elapsed();
    println!(
        "| Semilla | Estimacion | Varianza | Intervalo | Tiempo | Repeticiones |
       |----|----|----|----|----|----|
        | {} | {} | {} | [{}, {}] | {:?} | {} |",
        24242, s_lebesgue, v_lebesgue, intevalo.0, intevalo.1, duration, repeticiones
    );
    let n_n = (n.inverse_cdf(0.975) as f32).powi(2) * v / 0.001_f32.powi(2);

    for i in 1..=10 {
        rng = rand::SeedableRng::seed_from_u64(23984238648 + i * 16);

        // Aplico metodo de montecarlo
        let mut s = 0.0;
        let mut t = 0.0;

        let start = Instant::now();
        for j in 1..=n_n as i32 {
            // Genero valores aleatorios
            let x = unif_distribution.sample(&mut rng);
            let y = unif_distribution.sample(&mut rng);

            let height = mountain_function(x, y);
            if j > 1 {
                t += (1.0 - 1.0 / j as f32) * (height - s / (j as f32 - 1.0)).powi(2);
            }
            s += height;
        }

        let s_lebesgue = s / n_n;
        let v = t / (n_n - 1.0);
        let v_lebesgue = v / n_n;
        let intevalo = (
            s_lebesgue - n.inverse_cdf(0.975) as f32 * (v / n_n).sqrt(),
            s_lebesgue + n.inverse_cdf(0.975) as f32 * (v / n_n).sqrt(),
        );

        let duration = start.elapsed();
        println!(
            "| {} | {} | {} | [{}, {}] | {:?} | {} |",
            23984238648 + i * 16,
            s_lebesgue,
            v_lebesgue,
            intevalo.0,
            intevalo.1,
            duration,
            n_n
        );
    }
}
