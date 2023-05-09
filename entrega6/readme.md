# I. Unidad 4 - Sesión 11 -  Ejercicio 11.1

# II. Trabajo Individual.

Bruno De Simone, CI 49145550, bruno.de.simone@fing.edu.uy

# III. Descripción del Problema
```
Problema: se idealiza una montaña como un cono inscrito en una región cuadrada de lado 1 km. La base de la montaña es circular, con centro en (0.5, 0.5) y radio r = 0.4km, y la altura es H = 8km. La altura de cada punto (x, y) de la montaña está dada por la función f(x, y) = H − H/r × ((x − 0.5)^2 + (y − 0.5)^2)^(1/2), en la zona definida por el círculo, y 0 fuera del círculo. El volumen total de la montaña (en km cúbicos) puede verse como la integral de la función altura en la región.
```
- **Parte a**: escribir un programa para calcular el volumen por Monte Carlo. Realizar 10^6 replicaciones y estimar el valor de ζ y el error cometido (con nivel de confianza 0.95), utilizando como criterio la aproximación normal

# IV. Descripción de la Solución

## Parte a)
La forma de resolver esta parte fue tomando el problema cómo estimar el valor la integral en el sentido de Lebesgue mediante método de montecarlo, la función a integral es la dada para calcular la altura de un punto de la montaña. Ademas de esto se modifico el algoritmo utilizado en el ejercicio 6.1 para generar unicamente puntos dentro del circulo de la montana.

Para generar un punto aleatorio (X1, X2) en un circulo de centro (0.5, 0.5) y
radio 0.4, es posible hacerlo de la forma siguiente (derivacion disponible en
las paginas 234 y 235 del libro de referencia del curso, “Monte Carlo:
concepts, algorithms and applications”, Fishman 1996):

* se genera un valor aleatorio r, de distribucion Fr(x) = x^2 para 0 ≤ x ≤ 1, y 0 para cualquier otro x;
* se generan dos v.a. independientes Z1 y Z2 de distribucion normal (0, 1);
* se calcula:
  * X1 = r * Z1 * 0.4 / (Z1 ^ 2 + Z2 ^ 2) ^ 1/2 + 0.5
  * X2 = r * Z2 * 0.4 / (Z1 ^ 2 + Z2 ^ 2) ^ 1/2 + 0.5


```rust
// nueva funcion para generar puntos aleatorios dentro del circulo
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
```

```rust
for j in 1..=repeticiones {
    // Genero valores aleatorios
    --- antes
    let x = unif_distribution.sample(&mut rng);
    let y = unif_distribution.sample(&mut rng);
    ---
    let (x, y) = generar_punto_en_circulo(unif_distribution, normal_distribution, &mut rng);
    --- despues

    // Obtener valor de la función en el punto
    let height = mountain_function(x, y);

    // Actualizo los acumuladores
    if j > 1 {
        t += (1.0 - 1.0 / j as f32) * (height - s / (j as f32 - 1.0)).powi(2);
    }
    s += height;
}

let f_rep = repeticiones as f32;

// Estimacion
let s_lebesgue = (s / f_rep) * area_circulo;
// Varianza puntual de la función
let v = t / (f_rep - 1.0);
// Varianza de la estimación
let v_lebesgue = v / f_rep;
```

Luego el cálculo del intervalo de confianza se hizo mediante la aproximación normal (confianza de 95%).
```rust
let intervalo = (
    s_lebesgue - n.inverse_cdf(0.975) as f32 * (v / f_rep).sqrt(),
    s_lebesgue + n.inverse_cdf(0.975) as f32 * (v / f_rep).sqrt(),
);
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

## Parte a)

| Ej | Semilla | Estimación | Varianza | Intervalo | Tiempo |
       |----|----|----|----|----|----|
| 6.1 | 24242 | 1.3385398 | 0.0000035470912 | [1.3348485, 1.3422312] | 32.021715ms |
| 11.1 | 24242 | 1.3409284 | 0.0000035559813 | [1.3372325, 1.3446244] | 67.496965ms |

Vemos que ambas estimaciones son cercanas, se esperaba que la varianza en el nuevo metodo modificado sea menor, sin embargo ocurrio lo contrario. Ademas vemos como el tiempo empleado para calcular la estimacion se duplico. Consideramos que para un numero tan elevado de repeticiones no hace mucha diferencia este metodo, sin embargo para un numero menor de repeticiones puede ayudar el hecho de generar puntos dentro del circulo y ser una buena opcion.

### Ejecución
el comando de ejecución para el binario compilado es:
```bash
Usage: entrega6 [OPTIONS]

Options:
  -r, --repeticiones <REPETICIONES>  cantidad de replicaciones 'n' a realizar [default: 1000000]
  -h, --help                         Print help
```
