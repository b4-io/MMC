use clap::Parser;
use rand::{distributions::*, rngs::StdRng};
use statrs::distribution::{ContinuousCDF, Normal};
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    /// cantidad de replicaciones 'n' a realizar
    #[arg(short, long, default_value = "1000000")]
    repeticiones: i32,
}

const CENTER: (f32, f32) = (0.5, 0.5);
const RADIUS: f32 = 0.4;
const HEIGHT: f32 = 8.0;

fn main() {
    let args = Args::parse();

    mmc(args.repeticiones);
}

fn generar_punto_en_circulo(
    unif_distribution: Uniform<f32>,
    normal_distribution: Normal,
    rng: &mut StdRng,
) -> (f32, f32) {
    let r = unif_distribution.sample(rng).sqrt();
    let z1 = normal_distribution.sample(rng) as f32;
    let z2 = normal_distribution.sample(rng) as f32;

    let x1 = ((r * z1 * RADIUS) / (z1.powi(2) + z2.powi(2)).sqrt()) + CENTER.0;
    let x2 = ((r * z2 * RADIUS) / (z1.powi(2) + z2.powi(2)).sqrt()) + CENTER.1;

    (x1, x2)
}

fn mmc(repeticiones: i32) {
    // Creo funciones distribucion
    let unif_distribution = Uniform::new_inclusive(0.0, 1.0);
    let normal_distribution = Normal::new(0.0, 1.0).unwrap();

    //let mut rng = rand::thread_rng();
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(24242);

    let mountain_function = |x: f32, y: f32| {
        HEIGHT - (HEIGHT / RADIUS) * ((x - CENTER.0).powi(2) + (y - CENTER.1).powi(2)).sqrt()
    };

    // Aplico metodo de montecarlo
    let mut s = 0.0;
    let mut t = 0.0;

    let start = Instant::now();

    for j in 1..=repeticiones {
        // Genero valores aleatorios
        let (x, y) = generar_punto_en_circulo(unif_distribution, normal_distribution, &mut rng);

        // Obtengo valor de la funcion en el punto
        let height = mountain_function(x, y);

        // Actualizo los acumuladores
        if j > 1 {
            t += (1.0 - 1.0 / j as f32) * (height - s / (j as f32 - 1.0)).powi(2);
        }
        s += height;
    }

    let f_rep = repeticiones as f32;
    let area_circulo = RADIUS.powi(2) * std::f32::consts::PI;

    // Estimacion
    let s_lebesgue = (s / f_rep) * area_circulo;
    // Varianza puntual de la funcion
    let v = t / (f_rep - 1.0);
    // Varianza de la estimacion
    let v_lebesgue = v / f_rep;

    let intevalo = (
        s_lebesgue - normal_distribution.inverse_cdf(0.975) as f32 * (v / f_rep).sqrt(),
        s_lebesgue + normal_distribution.inverse_cdf(0.975) as f32 * (v / f_rep).sqrt(),
    );

    let duration = start.elapsed();
    println!(
        "| Semilla | Estimacion | Varianza | Intervalo | Tiempo | Repeticiones |
       |----|----|----|----|----|----|
        | {} | {} | {} | [{}, {}] | {:?} | {} |",
        24242, s_lebesgue, v_lebesgue, intevalo.0, intevalo.1, duration, repeticiones
    );
}
