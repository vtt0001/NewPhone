# Marco de pruebas

Este documento contiene el análisis previo a la elección de un marco de pruebas que nos permita lanzar los test y ver los resultados de los mismos de una forma rápida y sencilla.

---
## Introducción

Hemos encontrado dos opciones interesantes a la hora de seleccionar un marco de pruebas para nuestro proyecto:

- Cargo test

- Stainless

Las documentaciones de cada uno de los marcos están linkadas en los comentarios del [issue 16](https://github.com/vtt0001/NewPhone/issues/16)

---
## Posibles Marcos
---
### Cargo test:

La primera ventaja que encontramos a la hora de usar cargo test es que es nativa de cargo, es decir, una vez instalado cargo tendremos cargo test también instalado.

El uso de cargo test es muy simple, nos permite ejecutar todos los test mediante:

<code> $ cargo test </code>

También podemos ejecutar test cuyo nombre coincida con un patrón:

<code> $ cargo test < patrón > </code>

Si sustituimos <code> < patrón > </code> por <code> < nombreTest > </code> estaremos ejecutando un test concreto.

Tenemos que tener en cuenta, como una ventaja más de cargo test, que es capaz de ejecutar test de forma concurrente lo que lo hace un marco de pruebas rápido en cuanto a tiempos de ejecución de un conjunto de test.

### Stainless:

Este marco presenta un inconveniente crucial que se puede leer en las primeras líneas de la documentación oficial: "Note that stainless currently requires the nightly version of the Rust compiler!", es decir, no es un marco de pruebas que podamos usar en todas las versiones de rust. Tenemos que tener también en cuenta que no es nativo y por lo tanto precisa de una instalación específica para su uso.

Como ventaja a destacar, stainless permite generar rápidamente jerarquías de prueba complejas y reducir el texto estándar.

Por último, indicar que este marco de prueba no permite declaraciones use dentro de los bloques describe!, algo que no deja de ser una desventaja.



---
## Conclusiones y elección de biblioteca:
---

Tras ver ambas opciones parece bastante evidente que, con el simple hecho de que stainless no se puede usar con todas las versiones de rust, no será la opción más adecuada. Cargo test es la mejor opción por su facilidad de uso y disponibilidad en todas las versiones de rust. Por lo tanto, el marco de pruebas seleccionado para nuestro proyecto es cargo test.