# Cuatro aspectos de Rust que considero muy útiles 

En Rust son de especial utilidad porque son intrínsecos al lenguaje y el compilador se encarga de asegurarse que se cumplen. 

- Por ejemplo, si una variable es de tipo Option<T> es imposible utilizarla sin procesar explícitamente el caso de que tenga un valor Some<T> y el caso de que no lo tenga None().

- Por ejemplo, si una función devuelve un Result<T, E> es imposible llamarla sin procesar explícitamente el caso de resultado correcto Ok<T> y el caso de error Err<E>

- Por ejemplo, si variable es de tipo Enum (indicando los posibles valores válidos), es imposible comprobar cual valor tiene en un momento dado sin procesar explícitamente todos y cada uno de dichos posibles valores.

Cuando digo 'es imposible' es porque el compilador de Rust ni siguiera compila el código fuente si nos olvidamos de alguna parte. Eso sí, muy amablemente, nos recuerda la parte que nos falta... ;-)

## Managing null values (Option)

Rust tiene una manera de indicar que una variable puede estar vacia (sin valor).

El tipo de dato Option<T> puede contener:
- o un valor Some()
- o nada None()

Se acabaron los problemas con los null...

## Managing recoverable errors (Result)

Rust tiene una manera de indicar que una función puede fallar algunas veces y no devolver el valor que se espera.

El tipo de dato Result<T, E> puede contener:
- o un valor OK()
- o un error Err()

Se acabaron los throw...  try...catch para lidiar con problemas que se pueden producir habitualmente y son esperables.

## Pattern matching (match)

Rust implementa la manera funcional de comprobar el valor de algo y lidiar con todos los posibles casos que nos interesen.

No solo para comprobar valores en Enums o en Options o en Results, sino para cualquier tipo de comprobaciones en cualquier rango de valores.

Se acabaron los apilamientos de if...else encadenados.

Con la ventaja además de que, si el compilador sabe de antemano qué rango de valores puede haber, nos avisa cuando en el "matching" se nos haya olvidado tratar con alguna de las posibilidades.

## Processing a series of items (filter, map, reduce)

Rust implementa la manera funcional de trabajar con cualquier colección de valores, bien sea una lista, un array, una tupla, un diccionario, un rango,... (cualquier colección que disponga de un iterador).

- Para extraer una nueva lista con los valores que cumplan una cierta condición, tenemos `filter`.

- Para generar una nueva lista haciendo algo (una operación) a todos y cada uno de los valores, tenemos `map`.

- Para hacer algo (una operación) a todos y cada uno de los valores, llevando un acumulador y devolviendo un solo valor tras el proceso, tenemos `reduce`.
 
Se acabaron los bucles `for` para recorrer toda la colección...

nota: aquí he exagerado un poco ;-) los bloques `for` y `foreach` también se siguen utilizando en Rust, ya que hay situaciones donde siguen siendo útiles.

### expresiones lambda

Comentar que la forma de indicar cómo filtrar o cómo operar, suele ser utilizando expresiones lambda: una pequeña expresión o mini-función donde se indica simplemente qué valores se tienen en cuenta y qué se ha de hacer con ellos. 

Habitualmente se escriben con el formato: entrada => salida 

En Rust se escriben con el formato: |entrada| salida

