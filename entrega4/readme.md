# I. Unidad 2 - Sesión 7 -  Ejercicio 1

# II. Trabajo Individual.

Bruno De Simone, CI 49145550, bruno.de.simone@fing.edu.uy

# III. Descripción del Problema
```
Problema: supongamos que un programa de posgrado incluye un conjunto de P profesores y otro de S estudiantes, y que se desea asignar a cada estudiante un profesor para consultas, pero que todas las personas son de diferentes paıses y comprenden distintos idiomas. Si cada estudiante tiene asignado un profesor para consultas, la cantidad total de formas de asignar profesores a los estudiantes es P^ S (hay P opciones para cada estudiante, y S estudiantes en total, la cantidad total de opciones es el producto sobre los estudiantes de las opciones de cada estudiante). Supongamos que nos interesa determinar cuantas formas hay de asignar profesores a los estudiantes, respetando que cada profesor asignado a un estudiante tenga un idioma en com´un con el (para que puedan comunicarse). Si hay L lenguajes posibles, podemos tener para cada estudiante s un subconjunto Id(s) con los lenguajes que entiende, y lo mismo para cada profesor p; y una asignacion solo serıa valida si para cada estudiante s y su profesor p(s), se cumple que Id(s) ∩ Id(p(s)) != ∅. Para estimar la cantidad de formas distintas de realizar estas asignaciones de profesores y estudiantes, es posible aplicar el metodo Monte Carlo.

Se debe recibir en entrada el numero de replicaciones a realizar, y el nivel de confianza; en salida, se debe dar la estimacion del numero de combinaciones NC, ası como la desviacion estandar y un intervalo de confianza (del nivel especificado) calculado en base al criterio de Agresti-Coull.
```
- **Parte a**: Parte a: escribir un programa para hacer el calculo previamente descrito. Entregar seudocodigo y codigo.

- **Parte b**: Dado un grupo de estudiantes y profesores cada uno con sus lenguages que comprender. Usando el programa anterior, y empleando 1000 replicaciones de Monte Carlo, estimar los valores de cuantas combinaciones NC hay para asignar profesores a estudiantes y que tengan un idioma en com´un, con intervalos de confianza de nivel 95%.

- **Parte c**: adaptar el programa para calcular el numero de combinaciones si ademas queremos agregar como restriccion que ningun profesor atienda menos de un estudiante ni mas de cuatro estudiantes. Repetir parte b con nuevas restricciones.

# IV. Descripción de la Solución

## Parte a)
Se implemento Monte-Carlos para espacios discretos como fue visto en la unidad 7. Se tiene una generador Uniforme que retorna valores entre 0 y la cantidad de profesores -1.

```rust
// Acumulador de combinaciones positivas
let mut x: f32 = 0.0;

// Itero cantidad de repeticiones
for _ in 0..repeticiones {
    // Para las limitacions de la parte c
    let mut profesores_cantidad_asignado = Vec::from([0, 0, 0, 0]);
    // Flag que indica si se genero una asignacion valida en esta repeticion
    let mut valid = true;
    // Genero para todos los alumnos el numero de profesor asignado
    for lenguages in estudiantes.iter() {
        // Asigno profesor a alumno
        let profesor = uniform_dist.sample(&mut rng);

        // Acumulo alumno para parte c
        profesores_cantidad_asignado[profesor as usize] += 1;

        // Verifico que tengan lenguajes compatibles
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

    // Si no es valido siguiente repe
    if !valid {
        continue;
    }

    // en caso de ser parte c verfico extra checks
    if restricciones {
        // Cada profesor debe tener al menos 1 alumno y no mas de 4
        if profesores_cantidad_asignado[0] < 1 || profesores_cantidad_asignado[0] > 4 {
            continue;
        }
    }

    // Acumulo
    x += 1.0;
}

let f_rep = repeticiones as f32;

// r es P^S todas las asignaciones posibles (ver letra)
let r = (profesores.len() as f32).powi(estudiantes.len() as i32);

// Estimacion
let estimacion = r * x / f_rep;
// Varianza puntual de la función
let varianza = estimacion * (r - estimacion) / (f_rep - 1.0);
// Desviacion estandar
let desviacion_estandar = varianza.sqrt();
```

Luego el cálculo del intervalo de confianza se hizo mediante Agresti-Coull (confianza de 95%).
```rust
let n = Normal::new(0.0, 1.0).unwrap();
let delta: f64 = 0.05;

let z = n.inverse_cdf(1.0 - delta / 2.0) as f32;
let z_2 = z.powi(2);
let n_ = f_rep + z_2;
let x_ = x + z_2 / 2.0;
let p_ = x_ / n_;
let q_ = 1.0 - p_;

let interval = (
    r * (p_ - z * (p_ * q_).sqrt() * n_.powf(-0.5)),
    r * (p_ + z * (p_ * q_).sqrt() * n_.powf(-0.5)),
);
```

## Parte b)
Para as restricciones y el algoritmo los nombres de los estudiantes y profesores no importan por eso se definieron arrays de arrys con los lengujes de cada uno unicamente.
```rust
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

let profesores: Vec<Vec<&str>> = vec![
    /*Tom*/ vec!["Espanol", "Ingles", "Frances"],
    /*Luciana*/ vec!["Ingles", "Portugues"],
    /*Gerard*/ vec!["Frances", "Ingles"],
    /*Silvia*/ vec!["Espanol", "Frances"],
];
```
Luego se corrio el algoritmo de la parte A sin restricciones extra y 1000 iteraciones.

## Parte c)
Igual a la parte B pero con restricciones extras.

# V. Resultados Computacionales

Las pruebas fueron realizadas en una computadora con:
<ul>
<li>CPU: AMD Ryzen 7 1700x a 3.4 GHz</li>
<li>RAM: 16 gb DDR4 3200mhz</li>
<li>OS: PopOs (Ubuntu)</li>
<li>Semilla: 24242</li>
</ul>

![](./../pc.png)

## Parte b)
|  Epsilon  |    Delta    |    n_N  |
|:---------:|:-----------:|:---------:|
| 0.0001    | 0.05        | 184443973 |


## Parte c)

| Semilla | Estimación | Varianza | Intervalo | Tiempo | Repeticiones |
       |----|----|----|----|----|----|
| 23984238664 | 1.3363181 | 0.0000002407381 | [1.3353565, 1.3372798] | 258.84243ms | 13626005 |
| 23984238680 | 1.3360633 | 0.00000024065525 | [1.3351017, 1.3370248] | 251.860202ms | 13626005 |
| 23984238696 | 1.3359638 | 0.00000024055674 | [1.3350025, 1.3369251] | 253.701233ms | 13626005 |
| 23984238712 | 1.3364035 | 0.00000024078452 | [1.3354417, 1.3373653] | 250.7078ms | 13626005 |
| 23984238728 | 1.337034 | 0.00000024076755 | [1.3360723, 1.3379956] | 250.421143ms | 13626005 |
| 23984238744 | 1.3360319 | 0.0000002406933 | [1.3350704, 1.3369935] | 251.890569ms | 13626005 |
| 23984238760 | 1.3362976 | 0.0000002407636 | [1.335336, 1.3372593] | 252.167587ms | 13626005 |
| 23984238776 | 1.3364766 | 0.0000002405938 | [1.3355151, 1.337438] | 251.025332ms | 13626005 |
| 23984238792 | 1.3360193 | 0.0000002406802 | [1.3350577, 1.3369808] | 255.078916ms | 13626005 |
| 23984238808 | 1.3358076 | 0.0000002405703 | [1.3348463, 1.3367689] | 253.434424ms | 13626005 |

Vemos que entre todas las estimaciones no existe diferencia mayot a 0.002 que representa la cota hacia la derecha + la cota hacia la izquierda. Estaría bueno conocer el valor analitico del volumen de la montaña para poder obtener mayores conclusiones.

### Ejecución
el comando de ejecución para el binario compilado es:
```bash
Usage: entrega3 [OPTIONS]

Options:
  -r, --repeticiones <REPETICIONES>  cantidad de replicaciones 'n' a realizar [default: -1]
  -h, --help                         Print help
```
