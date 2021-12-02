# NewPhone. 

#### Estado: En fase de desarrollo

Aplicación que recomienda una lista de modelos de teléfonos móviles en función de las necesidades concretas de cada usuario y las características de los modelos.

---
## Integración continua, ¿Por qué?:

Teniendo en cuenta que es la primera vez que me enfrento a la implantación de integración continua en un proyecto, tomé la decisión de leer y documentarme sobre el tema ampliando los conocimientos explicados en clase.

La información extraída de por qué es una buena idea implantar integración continua es la siguiente:

- Reduce riesgos y tiempos.
- Reduce procesos manuales repetitivos automatizándolos.
- Crea una versión de tu software confiable, probado, versionado y repetible
- Mejora la visibilidad del estado del proyecto
- Mejora la calidad del producto
- Aumenta la facilidad de detección de errores o funcionamientos anómalos

## Travis

Travis es un sistema de integración continua que podemos configurar mediante el archivo [.travis.yml](https://github.com/vtt0001/NewPhone/blob/main/.travis.yml) alojado en la raíz del repositorio en cuestión.

Para ver la documentación asociada a travis, pulsar [aquí](https://github.com/vtt0001/NewPhone/blob/main/Doc/Integraci%C3%B3n%20continua/Travis.md)

## CircleCi:

CircleCi es un sistema de integración continua que podemos configurar mediante el archivo [config.yml](https://github.com/vtt0001/NewPhone/blob/main/.circleci/config.yml) alojado en el directorio .circleci.

Para ver la documentación asociada a CircleCi, pulsar [aquí](https://github.com/vtt0001/NewPhone/blob/main/Doc/Integraci%C3%B3n%20continua/CircleCi.md)


## Notas y aclaraciones

-**Nota 1:** En el [commit 5f3c0a5](https://github.com/vtt0001/NewPhone/commit/5f3c0a5d247675cc33a30fb34531cf0c116995df), se puede ver que se han aplicado ciertas mejoras al contenedor de nuestros test eliminando instrucciones innecesarias. Aclarar que también se ha desistido de la idea de ejecutar como root los test dentro del contenedor por los motivos que se indicaron en el hito anterior.

-**Nota 2:** En el [issue #46] Se puede observar que en la versión inicial de nuestro .travis.yml se estaba usando una instrucción deprecada, instrucción que además era redundante ya que estábamos asignando el valor que tiene por omisión. Es por este motivo que se eliminó del archivo.






