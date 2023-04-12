# I. Unidad 2 - Sesión 7 -  Ejercicio 1

# II. Trabajo Individual.

Bruno De Simone, CI 49145550, bruno.de.simone@fing.edu.uy

# III. Descripción del Problema
```
Problema: supongamos que un programa de posgrado incluye un conjunto de P profesores y otro de S estudiantes, y que se desea asignar a cada estudiante un profesor para consultas, pero que todas las personas son de diferentes países y comprenden distintos idiomas. Si cada estudiante tiene asignado un profesor para consultas, la cantidad total de formas de asignar profesores a los estudiantes es P^ S (hay P opciones para cada estudiante, y S estudiantes en total, la cantidad total de opciones es el producto sobre los estudiantes de las opciones de cada estudiante). Supongamos que nos interesa determinar cuántas formas hay de asignar profesores a los estudiantes, respetando que cada profesor asignado a un estudiante tenga un idioma en común con él (para que puedan comunicarse). Si hay L lenguajes posibles, podemos tener para cada estudiante s un subconjunto Id(s) con los lenguajes que entiende, y lo mismo para cada profesor p; y una asignación solo serıa valida si para cada estudiante s y su profesor p(s), se cumple que Id(s) ∩ Id(p(s)) != ∅. Para estimar la cantidad de formas distintas de realizar estas asignaciones de profesores y estudiantes, es posible aplicar el método Monte Carlo.

Se debe recibir en entrada el número de replicaciones a realizar, y el nivel de confianza; en salida, se debe dar la estimación del número de combinaciones NC, ası como la desviación estándar y un intervalo de confianza (del nivel especificado) calculado en base al criterio de Agresti-Coull.
```
- **Parte a**: escribir un programa para hacer el cálculo previamente descrito. Entregar pseudocódigo y código.

- **Parte b**: dado un grupo de estudiantes y profesores cada uno con sus lenguajes que comprender. Usando el programa anterior, y empleando 1000 replicaciones de Monte Carlo, estimar los valores de cuantas combinaciones NC hay para asignar profesores a estudiantes y que tengan un idioma en común, con intervalos de confianza de nivel 95%.

- **Parte c**: adaptar el programa para calcular el número de combinaciones si además queremos agregar como restricción que ningún profesor atienda menos de un estudiante ni más de cuatro estudiantes. Repetir parte b con nuevas restricciones.

# IV. Descripción de la Solución

## Parte a)
Se implementó Monte-Carlos para espacios discretos como fue visto en la unidad 7. Se tiene un generador Uniforme que retorna valores entre 0 y la cantidad de profesores -1.

```rust
// Acumulador de combinaciones positivas
let mut x: f32 = 0.0;

// Itero cantidad de repeticiones
for _ in 0..repeticiones {
    // Para las limitaciones de la parte c
    let mut profesores_cantidad_asignado = Vec::from([0, 0, 0, 0]);
    // Flag que indica si se genero una asignación válida en esta repetición
    let mut valid = true;
    // Genero para todos los alumnos el número de profesor asignado
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
    // Si no es válido siguiente repe
    if !valid {
        continue;
    }
    // en caso de ser parte c verificar extra checks
    if restricciones {
        // Cada profesor debe tener al menos 1 alumno y no más de 4
        if profesores_cantidad_asignado[0] < 1 || 
	        profesores_cantidad_asignado[0] > 4 {
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
// Desviación estándar
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
Para las restricciones y el algoritmo los nombres de los estudiantes y profesores no importan por eso se definieron arrays de arrays con los lenguajes de cada uno únicamente.
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
Luego se corrió el algoritmo de la parte A sin restricciones extra y 1000 iteraciones.

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

| Semilla | Estimación | Varianza | Desv Estand| Intervalo | Tiempo | Repeticiones |
       |----|----|----|----|----|----|
| 1848872944 | 105906.1 | 9993450 | 9996.72 | [87830.85, 127183.6] | 252.061µ | 1000 |


## Parte c)

| Semilla | Estimación | Varianza | Desv Estand| Intervalo | Tiempo | Repeticiones |
       |----|----|----|----|----|----|
| 1848872944 | 82837.5 | 8007945 | 8948.71 | [66867.984, 102185.6] | 131.295µ | 1000 |

Observamos que disminuye el número de combinaciones posibles estimadas, a primera vista esperaba que disminuyera mucho más el número.  El cambio de tiempo parece considerable y sin razón pero la realidad es que en números tan chicos de tiempo ese cambio es dado por diferencia en el sistema a la hora de ejecutar el binario y no por el binario en sí.

### Ejecución
el comando de ejecución para el binario compilado es:
```bash
Usage: entrega4 [OPTIONS]

Options:
  -n, --repeticiones <REPETICIONES>  cantidad de replicaciones 'n' a realizar [default: 1000]
      --r                            agrega restricciones de parte c
  -h, --help                         Print help
```
