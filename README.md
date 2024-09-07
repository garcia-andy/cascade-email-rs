# cascade-email-rs
![GitHub License](https://img.shields.io/github/license/garcia-andy/cascade-email-rs)

Una utilidad configurable para enviar correos a varios destinatarios a partir de un archivo de Excel.

## Instalación

```
cargo install cascade-email
```

## Debes configurar

- La configuración de correo en el archivo `data.json`
- El archivo `message.txt` con el mensaje de correo que se enviará a los usuarios
- El archivo `receivers.xlsx` con la tabla de datos de Excel

En el archivo `data.json` se debe especificar la configuración de la cuenta de correo.

```
{
    "mailer": {
        "domain": "gmail.com",
        "username": "agf030124",
        "password": "contraseña de acceso",
        "relay": "smtp.gmail.com",
        "remitente": "Andy Garcia",
        "subject": "Rust test email",
        "content": "lorem ipsum dolor sit amet"
    },
    "xlsx": {
        "name": "nombres_y_apellidos",
        "mail": "correo"
    }
}

El apartado "xlsx" contiene los nombres de las columnas de la tabla de datos de Excel, y el apartado "mailer" contiene la configuración de correo.

En el archivo `message.txt` se debe especificar el mensaje de correo que se enviará a los usuarios.
Utilizando la sintaxis de handlebars, se puede utilizar las variables de la tabla de datos de Excel.

Ejemplo:
```
Buenos días {{nombres_y_apellidos}} ({{correo}} - {{organismo_central}}),
le informamos que tu usuario {{usuario}} ha sido creado con éxito.
Por favor, utiliza la siguiente contraseña para acceder a tu cuenta:
{{contraseña}}
```

Por utlimo, se debe especificar el archivo `receivers.xlsx` con la tabla de datos de Excel.
