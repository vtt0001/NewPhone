# Integración continua con CircleCi:

Para poder hacer uso de CircleCI como sistema de integración continua en nuestro proyecto se han seguido los siguientes pasos:

- 1 Iniciar sesión en [circleCi.com](https://circleci.com/signup/) con la cuenta de github, esto se hará de un modo muy similar a como se hizo en travis.

- 2 Seleccionamos el repositorio sobre el que queremos incluir integración continua, en este caso, este mismo repositorio.

- 3 Incluir el archivo .circleci/config.yml en el repositorio:

![Archivo](https://github.com/vtt0001/NewPhone/blob/main/Img/Archivo%20CircleCi.png)

Lo primero que haremmos será indicar la versión de CircleCi más actual y estable. En este caso, 2.1.

Ahora vamos a hacer uso de nuestro contenedor para la ejecución de los test de nuestro código. Para ello indicamos la imagen de docker correspondiente.

Una vez hecho esto queda indicar los pasos a seguir, los cuales deben incluir la ejecución de los test.

**Aclaración**: No se usa el task manager para evitar tiempos excesivos, esto ya se justificó en el hito anterior al no incluirlo en el contenedor. Con travis, que no hace uso del contenedor, si que se ha tomado la decisión de usarlo, esta vez, preferimos mantener la decisión que tomamos anteriormente.

- 4 Comprobamos que todo ha funcionado bien:

![Check_OK](https://github.com/vtt0001/NewPhone/blob/main/Img/Check2%20OK.png)








