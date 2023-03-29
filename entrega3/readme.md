# I. Unidad 2 - Sesión 6 -  Ejercicio 6.1

# II. Trabajo Individual.

Bruno De Simone, CI 49145550, bruno.de.simone@fing.edu.uy

# III. Descripción del Problema
```
Problema: se idealiza una montaña como un cono inscrito en una región cuadrada de lado 1 km. La base de la montaña es circular, con centro en (0.5, 0.5) y radio r = 0.4km, y la altura es H = 8km. La altura de cada punto (x, y) de la montaña está dada por la función f(x, y) = H − H/r × ((x − 0.5)^2 + (y − 0.5)^2)^(1/2), en la zona definida por el círculo, y 0 fuera del círculo. El volumen total de la montaña (en km cúbicos) puede verse como la integral de la función altura en la región.
```
- **Parte a**: escribir un programa para calcular el volumen por Monte Carlo. Realizar 10^6 replicaciones y estimar el valor de ζ y el error cometido (con nivel de confianza 0.95), utilizando como criterio la aproximación normal

- **Parte b**: en base al valor estimado en la parte a, calcular el número de replicaciones necesario para obtener un error absoluto menor a 10^−3 (con nivel de confianza 0.95). 

- **Parte c**: realizar esa cantidad de replicaciones y estimar ζ y su intervalo de confianza. 

# IV. Descripción de la Solución

## Parte a)
La forma de resolver esta parte fue tomando el problema cómo estimar el valor la integral en el sentido de Lebesgue mediante método de montecarlo, la función a integral es la dada para calcular la altura de un punto de la montaña.

```rust
for j in 1..=repeticiones {
    // Genero valores aleatorios
    let x = unif_distribution.sample(&mut rng);
    let y = unif_distribution.sample(&mut rng);

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
let s_lebesgue = s / f_rep;
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

## Parte b)
Para obtener el cálculo del mínimo número de repeticiones que garanticen 95% de confianza con cota de error 0.001 según la aproximación normal se realizó el siguiente cálculo. Para realizar el cálculo de necesita la varianza puntual de la función obtenida en la parte a.

```rust
//(normal inversa de 0.975)^2 * varianza puntual de la función / cota de error^2
let n_n = (n.inverse_cdf(0.975) as f32).powi(2) * v / 0.001_f32.powi(2);
```

## Parte c)
Para esta parte se ejecutó código prácticamente igual al de la parte a pero con el nuevo número de repeticiones y semillas distintas.

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

| Semilla | Estimación | Varianza | Intervalo | Tiempo | Repeticiones |
       |----|----|----|----|----|----|
| 24242 | 1.3385398 | 0.0000035470912 | [1.3348485, 1.3422312] | 32.021715ms | 1000000 |

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
