# Gestor de tareas

Este documento contiene el análisis previo a la elección de un gestor de tareas adecuado en base a sus características

---
## Introducción

Son muchas las opciones de gestores de tareas que podemos usar, vamos a determinar aquí, de forma concienzuda, cuál de esas opciones vamos a usar.

Tras investigar las diferentes opciones disponibles para la elección de un gestor de tareas, centramos la elección en dos posibilidades:

- make

- cargo-make

---
## Posibles herramientas
---
### Make:

Una de las dos herramientas candidatas a ser usada como herramienta de gestor de tareas en el proyecto es make. En primer lugar, he preseleccionado esta herramienta porque ya la he usado y tengo conocimiento de la potencia de la misma.
Sabemos que make es un gestor de tareas nativo en todos los sistemas unix, además, es muy fácil de usar y ofrece la ventaja de ser uno de los gestores de tareas más usados, lo que nos indica que tendrá una comunidad activa considerable en caso de tener problemas o dudas con respecto a la herramienta.

Make es una herramienta portable y en general una buena opción a considerar.

---
### Cargo-make:

La segunda herramienta que vamos a contemplar como posible opción para ser usada como gestor de tareas de nuestro proyecto es cargo-make. 

Lo primero que cabe destacar de esta herramienta es que está escrita en rust para rust y como se indica en el artículo cuyo enlace puedes encontrar dirigiéndote al [issue 14](https://github.com/vtt0001/NewPhone/issues/14), en los comentarios, cargo-make da mucho valor añadido debido a que brinda características específicas de Rust.

Como sabemos, rust dispone de un task runner explícito llamado cargo y aunque cargo-make no sea una herramienta que venga incluida en el lenguaje, se puede considerar la herramienta stándar para rust a la hora de gestionar tareas dado que es parte de cargo.

Cargo-make también es una herramienta portable y aunque no sea nativo, es compatible con todos los sistemas Linux/Unix.

---
## Make vs cargo-make, decisión:

A pesar de que make no deja de ser una gran opción, ofreciendo además la característica de ser nativo en linux, que teniendo en cuenta que nuestro proyecto pretende ser desplegado en la nube es un punto a su favor, en este caso nos decantamos por cargo-make. La principal razón por la que se toma esta decisión es porque, al fin y al cabo, cargo-make es capaz de hacer prácticamete todo lo que es capaz de hacer make, además, incluye otras ventajas adicionales (compatible con secuencia de comandos en linea para varios lenguajes, permite tareas paralelas, permite scripts en línea y es extensible a otros archivos de cargo-make). En definitiva es una herramienta hecha específicamente para rust pero basada en make lo que la hace la opción más interesante.

---
## Instalando cargo-make

Instalar cargo-make, teniendo ya instalado cargo en nuestro sistema es algo muy sencillo. Bastará con ejecutar el siguiente comando:

<code>cargo install cargo-make</code>

Una vez instalado, pasamos a comprobar que si instaló correctamente como se puede ver en la imagen inferior:

![imageGT](https://github.com/vtt0001/NewPhone/blob/main/Img/cargo-make%20--version.png)

