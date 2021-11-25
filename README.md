# NewPhone. 

#### Estado: En fase de desarrollo

Aplicación que recomienda una lista de modelos de teléfonos móviles en función de las necesidades concretas de cada usuario y las características de los modelos.

---
## Elección de contenedor base:

En el [issue 26](https://github.com/vtt0001/NewPhone/issues/26) podemos ver la evolución que ha seguido la tarea de selección de contenedor base. Para acceder al documento de justificación pulsar [aquí](https://github.com/vtt0001/NewPhone/blob/main/Doc/Docker/Contenedor%20base.md).

## Dockerfile:

En el [issue 27](https://github.com/vtt0001/NewPhone/issues/27) podemos ver la evolución que ha seguido la tarea de generar mi dockerfile.

Tras leer sobre buenas prácticas en dockerfiles, se detectó la necesidad de reducir el contexto de construcción para minimizar el tamaño de la imagen, es por esto que se creó el [documento .dockerignore](https://github.com/vtt0001/NewPhone/blob/main/.dockerignore), el issue de esta tarea es [#28](https://github.com/vtt0001/NewPhone/issues/28).

Una vez hecho esto vimos la necesidad de poder descargar e instalar las dependencias como usuario sin privilegios de superusuario, para ello, hicimos uso de la herramienta cargo-chef. Esta herramienta se encarga de encapsular todo lo necesario para la construcción y almacenarlo en caché, de esta forma, el usuario no root podrá ejecutar los test de nuestra aplicación sin problemas. Esta tarea está descrita en el [issue #30](https://github.com/vtt0001/NewPhone/issues/30).

Aclaración: Nuestro dockerfile crea y destruye un main vacío para permitir a cargo-chef hacer la encapsulación.

Para ver el aspecto final de nuestro dockerfile pulsar [aquí](https://github.com/vtt0001/NewPhone/blob/main/dockerfile)

### Comentando el código:
![Dockerfile código](https://github.com/vtt0001/NewPhone/blob/main/Img/Dockerfile.png)

En primer lugar, usamos la imagen base alpine en su versión 3.14 e incluimos LABEL's con información sobre quien mantiene el código y la url del repositorio. Lo siguiente que hacemos es declarar algunas variables de entorno necesarias para instalar rust y cargo, nos colocamos en nuestro directorio /app/test del contenedor e indicamos root como usuario para proceder a instalar aquello que sea necesario. Las dos primeras órdenes COPY incluyen en el directorio los dos archivos cargo necesarios para que cargo chef pueda preparar y encapsular las dependencias, el último COPY guardará en el mismo directorio Makefile.toml para poder lanzar los test desde nuestro task manager.

Pasamos ahora a la instrucción RUN. Lo primero que haremos será actualizar, una vez hecho, creamos un directorio src con un archivo main vacío, esto, como ya se ha explicado antes, es un paso necesario para el correcto funcionamiento de cargo chef (será eliminado después). Ahora sí, instalamos todo aquello que necesitamos para poder correr los test de rust en alpine. Por último borramos el main.rs vacío y creamos un usuario sin permisos root, este será el encargado de hacer uso de makers test para ejecutar los test de nuestra aplicación.


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






