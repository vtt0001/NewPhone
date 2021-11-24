# Elección del registro alternativo.

Para buscar las opciones que mejor se adapten a las necesidades de un registro alternativo para nuestro proyecto, se han definido los siguientes citerios de busqueda:

- El registro debe ser público.
- El registro debe ser gratuito.
- El registro debe permitir la automatización de push mediante github actions.

Tras realizar una búsqueda de los registros que cumplen estas condiciones hemos decidido considerar las siguientes dos opciones:

- [Github container registry](https://docs.github.com/es/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- [Harbor](https://goharbor.io/docs/2.4.0/)

Una vez hemos podido observar con detenimiento ambos registros, se ha considerado que la mejor opción es usar github container registry no solo por su facilidad de uso y por coherencia en cuanto al marco de trabajo github si no, además, porque este registro de contenedores ofrece beneficios tales como permisos granulares y optimización de almacenamiento para las imágenes de Docker.

## Decisión final

Finalmente, concluimos seleccionando **github container registry** como registro alternativo para nuestro proyecto.



