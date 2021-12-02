# Integración continua con travis:

Para poder hacer uso de travis como sistema de integración continua en nuestro proyecto se han seguido los siguientes pasos:

- 1 Iniciar sesión en [travis-ci.com](https://www.travis-ci.com/) con la cuenta de github:

![Sing_in](https://github.com/vtt0001/NewPhone/blob/main/Img/Sing%20in%20travis.png)

- 2 Dar permisos a travis para acceder al repo:

![Permisos](https://github.com/vtt0001/NewPhone/blob/main/Img/Permisos%20travis.png)

- 3 Incluir el archivo travis.yml en mi repositorio:

![Archivo](https://github.com/vtt0001/NewPhone/blob/main/Img/Archivo%20travis.png)


Tras indicar el lenguaje de nuestro proyecto, almacenaremos en caché cargo, de esta forma, se reducirá drásticamente el tiempo que tarda una compilación en ejecutarse al almacenar en caché la compilación del proyecto como se indica en la [documentación de travis referente a este tema](https://docs.travis-ci.com/user/languages/rust/#dependency-management). Seguidamente, indicamos la rama que activará nuestro travis al recibir un push o pull request.

Ahora toca indicar que versiones de rust se van a testear. Debido a la decisión justificada más adelante de instalar cargo-make, cosa que incrementa considerablemente el tiempo de ejecución de travis, hemos decidido testear las dos últimas versiones estables y de rust, en este caso: 1.55.0 y 1.56.0.

Como se puede ver en el [issue #44](https://github.com/vtt0001/NewPhone/issues/44) se creó una primera versión sin usar cargo-make, posteriormente, se realizó la prueba de incluirlo para ver la diferencia de créditos gastado si usamos o no el task manager. Los resultados de la pruebas y conclusión fueron los siguientes:

Tras de incluir cargo-make, travis consume en cada ejecución **130 créditos**

:white_check_mark: Antes de la prueba:

Antes de incluir cargo-make, travis consumía en cada ejecución **40 créditos**

:heavy_check_mark: Después de la prueba:

Tras de incluir cargo-make, travis consume en cada ejecución **130 créditos**

Conclusiones:

A pesar de ser un incremento muy considerable, se ha tomado la decisión de usar cargo-make ya que es una buena práctica y se dispone de créditos suficientes para asumir el coste. **Habrá que lanzar travis solo en los casos estrictamente necesarios (usar [skip travis] )

Por último, hacemos uso del task manager para lanzar los test.

- 4 Comprobamos que todo ha funcionado bien a la vez de ver el tiempo y créditos consumidos en nuestra última versión de travis.yml:

![Check_OK](https://github.com/vtt0001/NewPhone/blob/main/Img/Check%20OK.png)








