# Cuatro aspectos que considero muy útiles 

En Rust estos aspectos son de especial utilidad porque son intrínsecos al propio lenguaje y el compilador se encarga de asegurarse que se cumplen. 

- Por ejemplo, si una variable es de tipo `Option<T>` es imposible utilizarla sin procesar explícitamente el caso de que tenga un valor (`Some<T>`) y el caso de que no lo tenga (`None`).

- Por ejemplo, si una función devuelve un `Result<T, E>` es imposible llamarla sin procesar explícitamente el caso de resultado correcto (`Ok<T>`) y el caso en que se ha producido un error (`Err<E>`)

- Por ejemplo, si una variable es de un tipo `Enum` (indicando los posibles valores válidos), es imposible comprobar su valor en un momento dado sin indicar explícitamente qué hacer en todos y cada uno los posibles casos que se puedan presentar.

Cuando digo 'es imposible' es porque el compilador de Rust ni siguiera compila el código fuente si no se ha programado correctamente. Eso sí, muy amablemente, nos suele recordar la parte que nos falta y nos suele sugerir posibles maneras de subsanarla.

>consejo: Leer atentamente todos los reportes de error del compilador. Leerlos completos, hasta el final. Son una verdadera mina de información.

## -1- Gestión de valores inexistentes (Option)

Rust tiene una manera expresa de indicar que una variable puede estar vacia (sin valor).

El tipo de dato `Option<T>` puede contener:
- o un valor Some()
- o nada None()

Se acabaron los problemas con los null...

(https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum-and-its-advantages-over-null-values)


## -2- Gestión de errores (Result)

Rust tiene una manera expresa de indicar que una función puede no devolver el valor que se espera.

El tipo de dato `Result<T, E>` puede contener:
- o un valor OK()
- o un error Err()

Se acabaron los throw...  try...catch para lidiar con problemas que se pueden producir habitualmente y son esperables.

(https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)


## -3- Pattern matching (match)

Rust implementa la manera funcional de comprobar el valor de algo y lidiar con todos los posibles casos que se puedan presentar.

No solo para comprobar valores en Enums o en Options o en Results, sino para cualquier tipo de comprobaciones en cualquier rango de valores.

Se acabaron los apilamientos de if...else encadenados.

Con la ventaja además de que, si el compilador sabe de antemano qué rango de valores puede haber, nos avisa cuando en el "matching" se nos haya olvidado tratar alguna de los posibles valores.

(https://doc.rust-lang.org/book/ch06-02-match.html)

(https://doc.rust-lang.org/book/ch18-00-patterns.html)


## -3- Trabajo con colecciones (filter, map, reduce)

Rust implementa la manera funcional de trabajar con colecciones de valores (listas, arrays, tuplas, diccionarios, rangos, o cualquier otra colección que disponga de un iterador).

- Para extraer una nueva lista con los valores que cumplan una cierta condición, tenemos `filter`.

- Para generar una nueva lista haciendo algo (una operación) a todos y cada uno de los valores, tenemos `map`.

- Para hacer algo (una operación) a todos y cada uno de los valores, llevando un acumulador y devolviendo un solo valor tras el proceso, tenemos `reduce`.
 
Se acabaron los bucles `for` para recorrer toda la colección...

nota: aquí he exagerado un poco ;-) los bloques `for` (`foreach`) también se siguen utilizando en Rust, ya que hay situaciones donde siguen siendo útiles.

[filter](https://doc.rust-lang.org/book/ch13-02-iterators.html#using-closures-that-capture-their-environment)

[map](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators)

[sum](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-consume-the-iterator)

[for](https://doc.rust-lang.org/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators)

[Comparing Performance: Loops vs. Iterators](https://doc.rust-lang.org/book/ch13-04-performance.html)

### expresiones lambda

Comentar que la forma de indicar cómo filtrar o cómo operar en expresiones `filter`, `map`, `reduce`,... suele ser utilizando expresiones lambda: una pequeña expresión o mini-función donde se indica simplemente qué valores se tienen en cuenta y qué se ha de hacer con ellos. 

En muchos lenguajes se escriben con el formato: `entrada => salida` 

En Rust se escriben con el formato: `|entrada| salida`

