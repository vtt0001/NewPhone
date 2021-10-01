# Configuración inicial:
---
Antes de comenzar a trabajar en el proyecto es necesario realizar algunas configuraciones previas referentes a git y github. A continuación se especifican las siguientes configuraciones:


## Creación del par claves pública y privada
Para tener una comunicación segura es necesario crear un par de claves pública/privada ssh que nos permita un acceso seguro a nuestra cuenta de github. Para ello el primer paso es crear dicho par de claves ssh como se muestra en la imagen a continuación: 
![imageClave](https://github.com/vtt0001/NewPhone/blob/main/Img/clave.png)

Una vez hemos creado las claves e incluidas al agente con el comando eval ***$(ssh-agent -s)*** incluimos la clave pública en github siguiendo el camino: *settings > SSH and GPG keys > new SSH key*. Como podemos comprobar en la siguiente imagen, ya hemos establecido una conexión segura: 
![imageClave2](https://github.com/vtt0001/NewPhone/blob/main/Img/clave2.png)


## Activación de 2FA

Para activar la autenticación de doble factor iremos por el siguiente flujo: *settings > Account security > Two-factor authentication*, seguimos los pasos que nos indican y finalmente nos indicará la correcta activación del 2FA como se muestra en la imagen:
![image2FA](https://github.com/vtt0001/NewPhone/blob/main/Img/2FA.png)


## Configuración del perfil

Lo último que haremos será configurar el nombre y el correo en la configuración de git, además incluiremos información relevante al perfil de github tales como nombre completo, avatar, ciudad y universidad.

### Configuración de nombre y correo:

![imageN-C](https://github.com/vtt0001/NewPhone/blob/main/Img/Nombre-correo.png)

### Configuración del perfil de github:

![image2FA](https://github.com/vtt0001/NewPhone/blob/main/Img/Perfil%20GH.png)