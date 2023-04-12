use clap::{command, Parser};
use std::time::Instant;

use rand::{distributions::Uniform, prelude::Distribution, rngs::StdRng};

#[derive(Parser, Debug)]
#[command(author)]
struct Args {
    /// cantidad de replicaciones 'n' a realizar
    #[arg(short = 'n', long, default_value = "1000")]
    repeticiones: i32,

    /// agrega restricciones de parte c
    #[arg(long = "r")]
    restricciones: bool,
}

fn main() {
    let args = Args::parse();
    let uniform_dist = Uniform::new_inclusive(0.0, 4.0);

    let repeticiones = args.repeticiones;
    let restricciones = args.restricciones;

    // let Estudiantes: HashMap<&str, Vec<&str>> = HashMap::from([
    //     ("Maria", vec!["Espanol", "Ingles"]),
    //     ("Sophie", vec!["Frances", "Ingles"]),
    //     ("Liliana", vec!["Espanol", "Portugues"]),
    //     ("Lucia", vec!["Ingles", "Portugues"]),
    //     ("Monique", vec!["Frances"]),
    //     ("Rodrigo", vec!["Espanol", "Ingles", "Frances"]),
    //     ("John", vec!["Ingles"]),
    //     ("Neymar", vec!["Ingles", "Portugues"]),
    //     ("Jaque", vec!["Frances", "Portugues"]),
    //     ("Juan", vec!["Espanol"]),
    // ]);

    let estudiantes: Vec<Vec<&str>> = vec![
        /*Maria*/ vec!["Espanol", "Ingles"],
        /*Sophie*/ vec!["Frances", "Ingles"],
        /*Liliana*/ vec!["Espanol", "Portugues"],
        /*Lucia*/ vec!["Ingles", "Portugues"],
        /*Monique*/ vec!["Frances"],
        /*Rodrigo*/ vec!["Espanol", "Ingles", "Frances"],
        /*John*/ vec!["Ingles"],
        /*Neymar*/ vec!["Espanol", "Portugues"],
        /*Jauqe*/ vec!["Frances", "Portugues"],
        /*Juan*/ vec!["Espanol"],
    ];

    // let Profesores: HashMap<&str, Vec<&str>> = HashMap::from([
    //     ("Gerard", vec!["Frances", "Ingles"]),
    //     ("Tom", vec!["Espanol", "Ingles", "Frances"]),
    //     ("Luciana", vec!["Ingles", "Portugues"]),
    //     ("Silvia", vec!["Espanol", "Frances"]),
    // ]);

    let profesores: Vec<Vec<&str>> = vec![
        /*Tom*/ vec!["Espanol", "Ingles", "Frances"],
        /*Luciana*/ vec!["Ingles", "Portugues"],
        /*Gerard*/ vec!["Frances", "Ingles"],
        /*Silvia*/ vec!["Espanol", "Frances"],
    ];

    //let mut rng = rand::thread_rng();
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(1848872944);

    let start = Instant::now();

    // Calulate 6 dimension volume using Monte Carlos Method
    let mut x: f32 = 0.0;
    for _ in 0..repeticiones {
        // Genero para todos los alumnos el numero de profesor asignado
        let mut profesores_cantidad_asignado = Vec::from([0, 0, 0, 0]);
        let mut valid = true;
        for lenguages in estudiantes.iter() {
            let profesor = uniform_dist.sample(&mut rng);
            profesores_cantidad_asignado[profesor as usize] += 1;

            // Si el profesor no puede dar el lenguaje, se repite el proceso
            let mut is_valid = false;
            for lenguaje in lenguages.iter() {
                if profesores[profesor as usize].contains(lenguaje) {
                    is_valid = true;
                    break;
                }
            }

            if !is_valid {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        if restricciones {
            // Cada profesor debe tener al menos 1 alumno y no mas de 4
            if profesores_cantidad_asignado[0] < 1 || profesores_cantidad_asignado[0] > 4 {
                continue;
            }
        }

        x += 1.0;
    }

    let f_rep = repeticiones as f32;
    let r = (profesores.len() as f32).powi(estudiantes.len() as i32);
    let estimacion = r * x / f_rep;
    let varianza = estimacion * (r - estimacion) / (f_rep - 1.0);
    let desviacion_estandar = varianza.sqrt();

    let duration = start.elapsed();

    println!(
        "Estimacion: {}, Varianza: {}, Desviacion Estandar: {}, Tiempo: {:?}",
        estimacion, varianza, desviacion_estandar, duration
    );
}
