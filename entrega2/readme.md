# I. Unidad 2 - Sesión 3 y 4

# II. Trabajo Individual.

Bruno De Simone, CI 49145550, bruno.de.simone@fing.edu.uy

# III. Descripción del Problema

## Ejercicio 3.1

```
Problema: Se desea estimar el volumen de una región R de [0, 1], 6 dimensiones, definida por todos los puntos de la hiper-esfera de centro (0.45, 0.5, 0.6, 0.6, 0.5, 0.45) y radio 0.35, que además cumplan las restricciones siguientes: 3x1 + 7x4 ≤ 5; x3 +x4 ≤ 1; x1 −x2 −x5 +x6 ≥ 0.
```
- **Parte a**: implementar un programa que reciba como parámetro la cantidad de replicaciones `n` a realizar, y emplee Monte Carlo para calcular (e imprimir) la estimación del volumen de R, y la desviación estándar de este estimador. Incluir código para calcular el tiempo de cálculo empleado por el programa. Utilizar el programa con n = 104 y luego con n = 106 para estimar el volumen de R. Discutir si los dos valores obtenidos parecen consistentes.

- **Parte b**: como forma de validar el programa, eliminar las restricciones adicionales de desigualdad, y comparar el volumen calculado por Monte Carlo con n = 10^6 con el valor exacto del volumen de una hiperesfera de dimensión 6. Discutir también la relación de este valor con el obtenido en la parte a.

## Ejercicio 4.1

- **Parte a**:  Comparar y discutir la dependencia de los criterios de peor caso nC, nN, nH frente a los parámetros ϵ y δ.

- **Parte b**:  Calcular nC, nN, nH para ϵ = 0.01, δ = 0.001, 0.01, 0.05.
# IV. Descripción de la Solución

## Ejercicio 3.1

### Parte a)
Se implementó Monte-Carlos para la estimación del volumen según las restricciones de la letra.

Primero se implementó una función que verifica si el punto generado pertenece a la hiperesfera definida.
```rust
fn belongs_to_hypersphear(x: f32, y: f32, z: f32, w: f32, u: f32, v: f32) -> bool {
    // Le resto para cada dimensión el valor del centro de la hiperesfera
    let x = x - 0.45;
    let y = y - 0.5;
    let z = z - 0.6;
    let w = w - 0.6;
    let u = u - 0.5;
    let v = v - 0.45;

    // Radio
    let r: f32 = 0.35;

    // Elevo al cuadrado los puntos
    let x = x.powi(2);
    let y = y.powi(2);
    let z = z.powi(2);
    let w = w.powi(2);
    let u = u.powi(2);
    let v = v.powi(2);

    let sum = x + y + z + w + u + v;

    // Verifico si pertenece
    sum <= r.powi(2)
}
```

Método de Monte-Carlos
```rust
// Acumulador de combinaciones positivas
let mut x: f32 = 0.0;
for _ in 0..repeticiones {
    // Genero un punto aleatorio en 6 dimensiones [0,1]
    let x1 = uniform_dist.sample(&mut rng);
    let x2 = uniform_dist.sample(&mut rng);
    let x3 = uniform_dist.sample(&mut rng);
    let x4 = uniform_dist.sample(&mut rng);
    let x5 = uniform_dist.sample(&mut rng);
    let x6 = uniform_dist.sample(&mut rng);

    // Chequeo que cumpla con todas las restricciones
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

    // Llamo a la función auxiliar para verificar que pertenece a la hiperesfera
    if !belongs_to_hypersphear(x1, x2, x3, x4, x5, x6) {
        continue;
    }

    // Acumulo
    x += 1.0;
}

let f_rep = repeticiones as f32;
// Estimacion
let estimacion = x / f_rep;
// Varianza
let varianza = estimacion * (1.0 - estimacion) / (f_rep - 1.0);
// Desviación estándar
let desviacion_estandar = varianza.sqrt();
```

### Parte b)

La estimación de la esfera se hace con el mismo algoritmo de la parte a pero con la flag `no_restricciones` en true.

## Ejercicio 4.1
### Parte a)
- **Chebyshec**: depende linealmente del valor de confianza buscado y en orden cuadrático de la cota. Esto significa que a menores cotas de errores el valor del peor caso va a aumentar cada vez más.

- **Teorema Central del Limite**: Es un método que depende principalmente de la cota de error pero por como es la ecuación se ve que el número de repeticiones es menor al que arroja **Chebyshev**  es entendible que se le llame un método optimista.

- **Teorema de Hoeffding**: es similar al teorema de Chebyshec en cuanto al orden de la dependencia  de epsilon, sin embargo depende de delta en orden logarítmico por lo que se pueden esperar menores números de repeticiones para deltas chicos.

### Parte b)

```rust
let n = Normal::new(0.0, 1.0).unwrap();
let delta: Vec<f64> = vec![0.001, 0.01, 0.05];
let epsilon: f64 = 0.01;

// Chebyshev worst case
let mut chebyshev = vec![];
for i in 0..delta.len() {
    chebyshev.push(1.0 / (4.0 * delta[i] * epsilon.powi(2)));
}

// LCT worst case
let mut lct = vec![];
for i in 0..delta.len() {
    lct.push((n.inverse_cdf(1.0 - delta[i] / 2.0) / (2.0 * epsilon)).powi(2));
}

// Hoeffding worst case
let mut hoeffding = vec![];
for i in 0..delta.len() {
    hoeffding.push(2.0 * (2.0 / delta[i]).ln() / (4.0 * epsilon.powi(2)));
}
```

# V. Resultados Computacionales

Las pruebas fueron realizadas en una computadora con:
<ul>
<li>CPU: AMD Ryzen 7 1700x a 3.4 GHz</li>
<li>RAM: 16 gb DDR4 3200mhz</li>
<li>OS: PopOs (Ubuntu)</li>
<li>Semilla: 24242</li>
</ul>

![](./../pc.png)

## Ejercicio 3.1
### Parte a)

| Semilla | Estimación | Varianza | Desv Estand| Tiempo | Repeticiones |
       |----|----|----|----|----|----|
| 1848872944 | 0.0003 | 2.999400e-8 | 1.731877e-4 | 280.404µs | 10000 |
| 1848872944 | 0.00028 | 2.79921e-10 | 1.673086e-5 | 27.99ms | 1000000 |

Vemos que ambas estimaciones son muy similares, aunque al estar tratando con un volumen tan chico la diferencia puede ser mayor a lo que se puede considerar una cota deseada. Sin intervalos de confianza es difícil obtener mayores conclusiones. Toda comparación de estimaciones debería estar sujeta a cota de error e intervalos de confianza. 
Otra cosa que debería probarse es el valor estimado con distintas semillas ya que puede justo haber usado una semilla buena en 10000 repeticiones.

### Parte b)

El valor real del volumen de la hiperesfera es 0.009499629.

| Semilla | Estimación | Varianza | Desv Estand| Tiempo | Repeticiones |
       |----|----|----|----|----|----|
| 1848872944 | 0.009567 | 9.475482e-9 | 9.734208e-5 | 18.09ms | 1000000 |

La estimación del volumen de la hiperesfera es bastante similar al valor analítico, diferencia de 0.000068. Mas allá de dar cierta confianza para ese número de repeticiones no saco conclusiones. 

## Ejercicio 4.1
### Parte b)
| Teorema | Epsilon |Deltas |  
       |----|----|----|----|----|
|  | 0.01 | 0.05 | 0.01 | 0.001| 
| Chebyshev |  |  49999.99999999| 249999.99999999|  2500000.0 |
| TCL |  |  9603.64705173| 16587.24150255|  27068.91542665 |
| Hoeffding |  |  18444.39727056| 26491.58683274|  38004.51229771 |

### Ejecución
el comando de ejecución para el binario compilado es:
```bash
Usage: entrega2 [OPTIONS] --ejercicio <EJERCICIO>

Options:
  -n, --repeticiones <REPETICIONES>  cantidad de replicaciones 'n' a realizar [default: 1000]
  -e, --ejercicio <EJERCICIO>        ejercicio a ejecutar (1 o 2)
      --nr                           eliminar (no) restricciones
  -h, --help                         Print help
```
