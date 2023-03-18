use std::time::Instant;

use rand::{distributions::Uniform, prelude::Distribution, rngs::StdRng};

pub fn ejercicio1(repeticiones: i32, no_restricciones: bool) {
    let uniform_dist = Uniform::new_inclusive(0.0, 1.0);

    //let mut rng = rand::thread_rng();
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(1848872944);

    let start = Instant::now();

    // Calulate 6 dimension volume using Monte Carlos Method
    let mut x: f32 = 0.0;
    for _ in 0..repeticiones {
        let x1 = uniform_dist.sample(&mut rng);
        let x2 = uniform_dist.sample(&mut rng);
        let x3 = uniform_dist.sample(&mut rng);
        let x4 = uniform_dist.sample(&mut rng);
        let x5 = uniform_dist.sample(&mut rng);
        let x6 = uniform_dist.sample(&mut rng);

        if !no_restricciones {
            //3x1 + 7x4 ≤ 5; x3 +x4 ≤ 1; x1 −x2 −x5 +x6 ≥ 0
            if 3.0 * x1 + 7.0 * x4 > 5.0 {
                continue;
            }
            if x3 + x4 > 1.0 {
                continue;
            }
            if x1 - x2 - x5 + x6 < 0.0 {
                continue;
            }
        }

        if !belongs_to_hypersphear(x1, x2, x3, x4, x5, x6) {
            continue;
        }

        x += 1.0;
    }

    let f_rep = repeticiones as f32;
    let estimacion = x / f_rep;
    let varianza = estimacion * (1.0 - estimacion) / (f_rep - 1.0);
    let desviacion_estandar = varianza.sqrt();

    let duration = start.elapsed();

    println!(
        "Estimacion: {}, Varianza: {}, Desviacion Estandar: {}, Tiempo: {:?}",
        estimacion, varianza, desviacion_estandar, duration
    );

    if no_restricciones {
        hypersphere_volume();
    }
}

// center (0.45, 0.5, 0.6, 0.6, 0.5, 0.45)
// radius 0.35
fn hypersphere_volume() {
    let r: f32 = 0.35;
    let v = (1.0 / 6.0) * r.powi(6) * std::f32::consts::PI.powi(3);
    println!("Volumen de la hipersfera: {}", v);
}

fn belongs_to_hypersphear(x: f32, y: f32, z: f32, w: f32, u: f32, v: f32) -> bool {
    let x = x - 0.45;
    let y = y - 0.5;
    let z = z - 0.6;
    let w = w - 0.6;
    let u = u - 0.5;
    let v = v - 0.45;

    let r: f32 = 0.35;

    let x = x.powi(2);
    let y = y.powi(2);
    let z = z.powi(2);
    let w = w.powi(2);
    let u = u.powi(2);
    let v = v.powi(2);

    let sum = x + y + z + w + u + v;

    sum <= r.powi(2)
}
