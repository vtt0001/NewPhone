# Elección del contenedor base

Ha llegado el momento de dockerizar los test de nuestra aplicación, para ello, la primera decisión técnica que debemos tomar es la elección de un contenedor base.


Las opciones que vamos a analizar y de las cuales saldrá nuestro contenedor base son:

- [ubuntu](https://hub.docker.com/_/ubuntu)
- [alpine](https://hub.docker.com/_/alpine)
- [rust](https://hub.docker.com/_/rust)
- [rust:< version >-alpine](https://hub.docker.com/_/rust)

Veamos ahora en detalle cada una de ellas:

## Posibles contenedores base:

### Ubuntu

Comenzamos inspeccionando el contenedor oficial de ubuntu, más concretamente su última versión 22.04. Si revisamos el [repositorio del contenedor](https://github.com/docker-library/repo-info/tree/master/repos/ubuntu) podemos ver el tamaño de dicho contenedor, en este caso tenemos un tamaño virtual de 76.15 Mb.

Una vez tenemos claro el tamaño de la imagen, es imprescindible, para poder tener en cuenta si se está manteniendo, revisar cuando se hizo el último commit sobre el repositorio. Para el contenedor que estamos analizando el último commit se hizo hace 6 días, lo que nos indica que es un contenedor que sigue manteniéndose como era de esperar.

En cuanto a las variables de entorno, podemos ver que el contenedor ubuntu nos proporciona la variable de entorno PATH, en ella se especifican las rutas en las cuales el intérprete de comandos (bash) debe buscar los programas a ejecutar.

### Alpine

Hablaremos ahora del contenedor alpine, más concretamente su última versión latest (3.14.3). Si revisamos el [repositorio del contenedor](https://github.com/docker-library/repo-info/tree/master/repos/alpine) podemos ver el tamaño de dicho contenedor, en este caso tenemos un tamaño virtual de 5.61 Mb.

Se ha podido comprobar que alpine es una imagen que está siendo mantenida actualmente, ya que, el último commit sobre el repositorio se hizo ayer.

Hablando ahora de las variables de entorno que nos proporciona alpine, como en el caso anterior, tenemos exclusivamente la variable PATH. En este caso, el intérprete de comandos es /bin/sh

### Rust

Seguimos con el contenedor rust, más concretamente su última versión latest (3.14.3). Si revisamos el [repositorio del contenedor](https://github.com/docker-library/repo-info/tree/master/repos/alpine) podemos ver el tamaño de dicho contenedor, en este caso tenemos un tamaño virtual de 1.26 Gb.

El último commit que se ha hecho sobre rust latest en el repositorio fue hace 15 días, lo que nos indica, como era de esperar, que la última versión de rust está siendo mantenida actualmente.

Las variables de entorno que nos proporciona este contenedor como contenedor base son:

- PATH
- RUSTUP_HOME
- CARGO_HOME
- RUST_VERSION

¿Qué son estas variables?:

##### PATH

Como en los casos anteriores se trata de una variable de entorno en la que se especifican las rutas en las cuales el intérprete de comandos (bash) debe buscar los programas a ejecutar.

##### RUSTUP_HOME

Como ya sabemos rustup nos permitirá gestionar la instalación y versiones de rust.

##### CARGO_HOME

Herramienta por defecto en rust que sirve para:

- Crear nuevos proyectos Rust a partir de templates (new, init)
- Compilar el proyecto actual (run, build, install)
- Gestionar dependencias del proyecto (search, update)
- Publicar el proyecto en crates.io (publish)
- Generar la documentación del proyecto (doc)
- Ejecutar los tests (test, bench)

Esta variable de entorno puede ser un punto a favor a tener en cuenta, ya que, nuestro proyecto hace uso de cargo a través del gestor de tareas. Véase [issue #22](https://github.com/vtt0001/NewPhone/issues/22).

##### RUST_VERSION

Variable de entorno con la versión que está instalada en el contenedor de rust, en este caso, 1.56.1.

### rust:< version >-alpine

Por último, hablaremos de una variante de la imagen rust basada en el proyecto Alpine Linux, más concretamente la versión 1.56-alpine3.14. Si revisamos el repositorio de rust mencionado en el apartado anterior, podemos ver el tamaño de la variante, en este caso tenemos un tamaño virtual de 797.54 Mb.

La imagen rust 1.56-alpine3.14 tiene su último commit hace 21 horas, lo que nos indica que también está siendo mantenida actualmente.

En cuanto a las variables de entorno, esta versión reducida del contenedor rust contiene exactamente las mismas que la opción anterior.

##Pruebas realizadas

Aclarar que solo se han realizado pruebas de las versiones oficiales, en este caso, rust y alpine ya que es una buena práctica hacer uso de las versiones oficiales del lenguaje o de contenedores muy reducidos como en el caso de alpine.

### Contenedor base alpine:

![Dockerfile](https://github.com/vtt0001/NewPhone/blob/main/Img/Dockerfile.png)

En el [issue 27](https://github.com/vtt0001/NewPhone/issues/27) podemos ver la evolución que ha seguido la tarea de generar mi dockerfile con alpine como base.

Vimos la necesidad de poder descargar e instalar las dependencias como usuario sin privilegios de superusuario, para ello, hicimos uso de la herramienta cargo-chef. Esta herramienta se encarga de encapsular todo lo necesario para la construcción y almacenarlo en caché, de esta forma, el usuario no root podrá ejecutar los test de nuestra aplicación sin problemas. Esta tarea está descrita en el [issue #30](https://github.com/vtt0001/NewPhone/issues/30).

Aclaración: Nuestro dockerfile crea y destruye un main vacío para permitir a cargo-chef hacer la encapsulación. Además, cabe destacar que hay malas prácticas incluidas en la prueba ya que fue la primera opción que decidimos probar y los conceptos aún no estaban bien asimilados.

### Contenedor base rust:

![Dockerfile_rust](https://github.com/vtt0001/NewPhone/blob/main/Img/Dockerfile_rust.png)

Como se puede observar esta imagen queda más limpia, el nivel de dificultad se reduce bastante y en general, parece compensar la limpieza y la facilidad a la hora de aplicar buenas prácticas fente al pequeño tamaño que incrementa con respecto al contenedor con base alpine.


## Tabla comparativa

| Característica | Ubuntu | Alpine | Rust | rust:< version >-alpine |
|:--------------:|:------:|:------:|:----:|:-----------------------:|
| Peso           |76.15 Mb|5.61 Mb|1.26 Gb|797.54 Mb|
| En catual mantenimiento|Si|Si|Si|Si|
| Variables de entorno|PATH|PATH|PATH<br/>RUSTUP_HOME<br/>CARGO_HOME<br/>RUST_VERSION|PATH<br/>RUSTUP_HOME<br/>CARGO_HOME<br/>RUST_VERSION|
| Intérprete de comandos|bash|bin/sh|bash|bash|
| Nivel de dificultad|-|Alto|Medio|-|


##Conclusiones y elección:

Teniendo en cuenta las pruebas realizadas y siendo el mayor punto de inflexión la facilidad con la que se puede montar el contenedor, la elección final para nuestro contenedor base será **rust:latest**. Asumimos usar un contenedor con algo más de peso ya que es imprescindible buscar un equlibrio entre tamaño y eficiencia a la hora de desarrollar.

