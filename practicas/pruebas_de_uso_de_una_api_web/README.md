A modo de prueba, se ha interactuado con una de las API de Autodesk.

[Premium Reporting API](https://aps.autodesk.com/en/docs/insights/v1/developers_guide/overview/)

y con la API auxiliar necesaria para acceder

[Authentication API](https://aps.autodesk.com/en/docs/oauth/v2/developers_guide/overview/)



## Para suministrar los secretos

- Para lanzar este programa desde un Powershell Windows  (un terminal de Visual Studio Code), se puede utilizar:

``` 
$env:CLIENT_ID="poner aquí el client id de la applicacion"; $env:CLIENT_SECRET="poner aquí el client secret de la applicacion"; $env:PAT="poner aquí un personal access token de Autodesk"; cargo run
```

- Para lanzar este programa desde un terminal Linux, se puede utilizar:

``` 
CLIENT_ID="poner aquí el client id de la applicacion" CLIENT_SECRET="poner aquí el client secret de la applicacion" PAT="poner aquí un personal access token de Autodesk" cargo run
``` 


Nota: se ha de escribir todo seguido, en una sola misma línea; sin saltos de línea.

