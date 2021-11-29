# NewPhone. 

#### Estado: En fase de desarrollo

Aplicación que recomienda una lista de modelos de teléfonos móviles en función de las necesidades concretas de cada usuario y las características de los modelos.

---
## Elección de contenedor base:

En el [issue 26](https://github.com/vtt0001/NewPhone/issues/26) podemos ver la evolución que ha seguido la tarea de selección de contenedor base. Para acceder al documento de justificación pulsar [aquí](https://github.com/vtt0001/NewPhone/blob/main/Doc/Docker/Contenedor%20base.md).

## Dockerfile:

Tras leer sobre buenas prácticas en dockerfiles, se detectó la necesidad de reducir el contexto de construcción para minimizar el tamaño de la imagen, es por esto que se creó el [documento .dockerignore](https://github.com/vtt0001/NewPhone/blob/main/.dockerignore), el issue de esta tarea es [#28](https://github.com/vtt0001/NewPhone/issues/28).

### Comentando el código:
![Dockerfile código](https://github.com/vtt0001/NewPhone/blob/main/Img/Dockerfile_rust.png)

En primer lugar, usamos la imagen base rust en su última versión e incluimos LABEL's con información sobre quien mantiene el código y la url del repositorio. Lo siguiente que hacemos es copiar Cargo.toml al contenedor para poder usar las dependencias. Creamos nuestro directorio app/test y creamos un main vacío que sirve para que rust deje compilar y posteriormente lanzar los test, incluimos un usuario noroot (aclarar que este paso no sería necesario, esta línea se ha mantenido así porque se pretende intentar lanzar el contenedor con dicho usuario en un futuro, para entender de lo que hablamos, ir al final de este documento, nota 2). Borramos Cargo.toml, cambiamos de directorio y ejecutamos los test. Como se puede observar no se hace uso del task manager, esta decisión queda justificada en la nota 3.


## Docker Hub y actualización automática:

Para subir la imagen de nuestro contenedor a Docker Hub y permitir la actualización automática del mismo se ha creado una github action [issue #32](https://github.com/vtt0001/NewPhone/issues/32). Esta github action hará un "build and push" de nuestro contenedor en Docker Hub. Para ver el código pulsar [aquí]

### Comentando el código:

![Docke Hub código](https://github.com/vtt0001/NewPhone/blob/main/Img/Github%20action%20Docker%20Hub.png)

Lo primero que haremos será dar un nombre, en este caso, Docker_Hub. Esta github action se ejecutará, como se puede ver en las líneas siguiente a name, cada vez que hagamos un push sobre la rama main y correrá sobre la última versión de ubuntu.

El cuerpo de nuestra github action está compuesto de tres pasos o step

- Extraemos los datos necesarios del repositorio para ejecutar los test en nuestro contenedor docker.

- Nos autenticamos en docker mediante docker/login-action en su última versión recomendada. Para esta tarea, ya que maneja datos sensibles, usaremos los secretos de github. Se puede ver la documentación de esto pulsando [aquí](https://docs.github.com/es/actions/security-guides/encrypted-secrets). Cabe destacar que github se ha preocupado de usar una [caja sellada de libsodium](https://libsodium.gitbook.io/doc/public-key_cryptography/sealed_boxes) para garantizar que los secretos llegan cifrados a github.
Podemos ver los secretos correctamente creados en la siguiente imagen.

![Secretos](https://github.com/vtt0001/NewPhone/blob/main/Img/Secretos.png)

- Por último solo nos queda hacer build and push, para ello github nos proporciona una action concreta que dado un contexto, en este caso el propio repositorio, teniendo la condición de push = true y proporcionándole el tag, en este caso nuestro nombre de usuario/ nombre del repositorio; construye el contenedor y hace push de dicho contenedor en Docker Hub.



## Registro alternativo:

En el [issue #35](https://github.com/vtt0001/NewPhone/issues/35) se puede ver la evolución de esta tarea. Lo primero que hicimos fue decidir que registro alternativo usar, para ver la justificación de la elección ir a [issue #36](https://github.com/vtt0001/NewPhone/issues/36) o directamente al documento que cierra dicho issue pulsando [aquí](https://github.com/vtt0001/NewPhone/blob/main/Doc/Docker/Registro%20alternativo.md).

Una vez decidido el registro alternativo, teniendo en cuenta que el código asociado a esta github action es muy similar al del apartado anterior, se dará explicación solo de las líneas de código que difieren.

![GHCR](https://github.com/vtt0001/NewPhone/blob/main/Img/GHCR.png)

La primera diferencia que podemos observar con respecto al código anterior son las variables de entorno que se han declarado. Estas variables de entorno nos servirán para después poder autenticarnos en GHCR. La variable REGISTRY contiene la dirección del registro, por otro lado, la variable IMAGE_NAME nos proporciona el nombre del repositorio.

Ahora vemos que al definir ubuntu para correr la github action incluimos los permisos de lectura para el contenido del repositorio y de escritura para los paquetes, que será lo que se creará.

Al autenticarnos, esta vez necesitamos especificar el registro, el usuario y un token. Este token no lo hemos creado nosotros, es un token automático de github que permite comprobar que estamos en el propio repositorio que vamos a usar.

Por último extraemos los metadatos necesarios construimos y hacemos push.

Tras realizar todo esto se nos creará un paquete en el repositorio. Para ver el paquete pulsar [aquí](https://github.com/vtt0001/NewPhone/pkgs/container/newphone).

## Notas y aclaraciones

-**Nota 1:** En el [issue #34] se puede ver que se ha incluido una nueva tarea en el documento [Makefile.toml](https://github.com/vtt0001/NewPhone/blob/main/Makefile.toml) para ejecutar el contenedor (task.docker_run)tal y como lo hará el profesor al corregir.

-**Nota 2:** Hablando con varios compañeros y con el propio profesor de la asignatura, llegamos a la conclusión de que la ejecución de test en rust con un usuario no root se puede llegar a hacer muy engorrosa, incluso haciendo uso de herramientas específicas como cargo-chef, que además, incrementan el tamaño del contenedor de una forma considerable. Es por esto que se tomó la decisión premeditada de no hacer el cambio de usuario a uno noroot. La decisión de dejar la creación del usuario viene impulsada por la perspectiva de seguir trabajando en la posibilidad de ejecutar como usuario no root.

-**Nota 3:** Se ha tomado la decisión de no usar el task manager para la ejecución de los test en el contenedor por dos motivos. El primero y más importante es que el contenedor base de rust tiene ya un peso bastante elevado como para instalar además cargo-make. Por otro lado, la tarea que debe ejecutar es muy simple, solo ejecuta <code> cargo test </code> y sería un error instalar cargo-make con el incremento de peso que conlleva para lanzar esa instrucción simple.






