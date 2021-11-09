# Biblioteca de aserciones

Este documento contiene el análisis previo a la elección de una biblioteca de aserciones adecuada en base a sus características

---
## Introducción

A la hora de escribir aserciones, tenemos dos grandes grupo o estilos de aserciones BDD vs TDD.

Veamos una tabla de diferencias clave entre TDD y BDD extraída del siguiente [artículo](https://www.softwaretestinghelp.com/tdd-vs-bdd/)

![imageTDDvsBDD](https://github.com/vtt0001/NewPhone/blob/main/Img/TDD%20vs%20BDD.png)

Una vez tenemos claro, gracias al artículo anteriormente mencionado y algunos otros encontrados en la red, las diferencias entre TDD y BDD, pasamos a analizar las bibliotecas de aserciones que barajamos para nuestro proyecto, las cuales son:

- Spectral

- Galvanic

- Speculoos

- Asert! macro

Todos los enlaces a la documentación de las diferentes bibliotecas de las cuales se ha extraído gran parte de la información se puede encontrar en los comentarios del [issue #15](https://github.com/vtt0001/NewPhone/issues/15)

---
## Posibles bibliotecas
---
### Spectral:

Para poder entender mejor como funciona spectral, vamos a ver un ejemplo muy sencillo de una aserción en la que nos basaremos para describir la biblioteca.

<code>assert_that(&subject).starts_with(&"H");</code>

Como se puede observar, spectral usa un lenguaje entendible que se acerca bastante al lenguaje natural. Esto, sin ninguna duda, puede ser interpretado como una ventaja a favor de la biblioteca, ya que teniendo conocimientos básicos de inglés, podemos entender que estamos queriendo comprobar (En el caso del ejemplo, comprobamos que el contenido de la variable subject empieza por el caracter 'H').

Siguiendo el mismo ejemplo, observemos como spectral nos muestra el mensaje de prueba fallida. Supongamos el siguiente código:

<code>
pub fn should_be_the_correct_string() {

    let subject = "ello World!";

    assert_that(&subject).starts_with(&"H");
}
</code>

![imageTDDvsBDD](https://github.com/vtt0001/NewPhone/blob/main/Img/Spectral%20fallo.png)

Como se observa, spectral indica de forma clara qué se esperaba y qué se ha obtenido.

Cabe remarcar que no es una biblioteca nativa o estándar, sin embargo, es una opción a tener en cuenta.

### Galvanic:

Al igual que en el caso anterior, galvanic también tiene un lenguaje bastante asequible y similar al lenguaje natural aunque quizás no tanto. Veamos otro ejemplo simple de una aserción en esta biblioteca:

<code> assert_that!(&Num != 1, otherwise "El número introducido es igual a 1");</code>

Este ejemplo nos lleva a una de las características que más llama la atención de esta biblioteca y es, la capacidad a través de la palabra reservada  otherwise, de poder usar mensajes de error en caso de que la asserción falle.

Esta biblioteca nos permite comprobar o chequear el tipo de dato de una variable a través de la orden is_variant!, evitando así el uso de if-let

<code>assert_that!(&ok, is_variant!(Ok));</code>

### Speculoos:

Tras revisar la documentación de spectral y después la de Speculoos, no se encuentran diferencias entre una y otra, parecen la misma biblioteca con diferente nombre. Se ha mirado un poco más a fondo y hemos llegado al repositorio de la biblioteca speculoos, se puede visitar dicho repositorio pulsando [aquí](https://github.com/oknozor/speculoos). En dicho repositorio, al principio del readme vemos el siguiente texto: "This is a fork the unmaintained crate spectral. Spectral as not changed for five years and yet is still very usable, the goal of this fork is to add new assertion capabilities without breaking the existing API."

Una vez entendido que speculoos es un fork de la biblioteca spectral sin mantenimiento, queda descartada como opción para ser usada como la biblioteca de aserciones para nuestro proyecto.

### Assert! macro

Como ventaja principal de esta biblioteca de aserciones tenemos la característica de que hablamos de una biblioteca de aserciones nativa de rust.

Vamos a ver que aspecto tiene una aserción simple haciendo uso de esta biblioteca:

<code>#[cfg(test)]

mod tests {

    #[test]

    fn it_works() {

        assert_eq!(2 + 2, 4);
    }
}
</code>

Podemos observar que la aserción de ejemplo es muy simple y solo comprueba que 2+2 sea igual a 4. Este sencillo ejemplo nos permite entender la simplicidad de la biblioteca, lo que representa una ventaja a la hora de ser elegida como biblioteca de aserciones para nuestro proyecto.

---
## Conclusiones y elección de biblioteca:
---

Tras haber hecho un estudio de las diferentes bibliotecas encontradas y teniendo en cuenta todo lo analizado de cada una de las opciones disponibles, se ha tomado la decisión de usar la biblioteca galvanic ya que, indagando más en la biblioteca vemos que tenemos mucha funcionalidad implementada que nos ahorrará mucho camino a la hora de crear test, además, su sintaxis es fácil e intuitiva.