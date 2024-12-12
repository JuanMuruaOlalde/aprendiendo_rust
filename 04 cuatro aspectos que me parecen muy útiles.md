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

(https://doc.rust-lang.org/std/option/index.html)

## -2- Gestión de errores recuperables (Result)

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

(https://doc.rust-lang.org/std/keyword.match.html)


## -3- Trabajo sobre colecciones (filter, map, reduce)

Rust implementa la manera funcional de trabajar con colecciones de valores (listas, arrays, tuplas, diccionarios, rangos, o cualquier otra colección que disponga de un iterador):

- Para extraer una nueva lista con los valores que cumplan una cierta condición, tenemos `filter`.

- Para generar una nueva lista haciendo algo (una operación) a todos y cada uno de los valores, tenemos `map`.

- Para hacer algo (una operación) a todos y cada uno de los valores, llevando un acumulador y devolviendo un solo valor tras el proceso, tenemos `reduce`.
 
Se acabaron los bucles `for` para recorrer toda la colección...

nota: aquí he exagerado un poco ;-) los bloques `for` (`foreach`) también se siguen utilizando en Rust, ya que hay situaciones donde siguen siendo útiles.

[filter](https://doc.rust-lang.org/book/ch13-02-iterators.html#using-closures-that-capture-their-environment)

[filter method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.filter)

[map](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators)

[map method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)

[sum](https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-consume-the-iterator)

[reduce method](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.reduce)

[for](https://doc.rust-lang.org/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators)

[Comparing Performance: Loops vs. Iterators](https://doc.rust-lang.org/book/ch13-04-performance.html)

### expresiones lambda

Comentar que la forma de indicar cómo filtrar o cómo operar en expresiones `filter`, `map`, `reduce`,... suele ser utilizando expresiones lambda: una pequeña expresión o mini-función donde se indica simplemente qué valores se tienen en cuenta y qué se ha de hacer con ellos. 

En muchos lenguajes se escriben con el formato: `entrada => salida` 

En Rust se escriben con el formato: `|entrada| salida`


## Estos aspectos en otros lenguajes


### Gestión de valores inexistentes

(https://en.wikipedia.org/wiki/Monad_(functional_programming)#An_example:_Maybe)

#### `C#`

[Custom Implementation of the Option/Maybe Type in C#](https://codinghelmet.com/articles/custom-implementation-of-the-option-maybe-type-in-cs)

[Embracing nullable reference types](https://devblogs.microsoft.com/dotnet/embracing-nullable-reference-types/)

[How to use the Either type](https://stackoverflow.com/questions/63231450/how-to-use-the-either-type-in-c/63282544#63282544)

[Nullable reference types](https://learn.microsoft.com/en-us/dotnet/csharp/nullable-references)

[Optional return in C#.Net](https://stackoverflow.com/questions/16199227/optional-return-in-c-net)

#### Java

[Class `Optional<T>`](https://docs.oracle.com/en/java/javase/20/docs/api/java.base/java/util/Optional.html)

[Optionals: Patterns and Good Practices](https://forums.oracle.com/ords/apexds/post/optionals-patterns-and-good-practices-2540)

[Guide To Java Optional](https://www.baeldung.com/java-optional)

[Uses for Optional in Java](https://www.baeldung.com/java-optional-uses)

[Using Optionals](https://dev.java/learn/api/streams/optionals/)

#### Python

[PEP 505 – None-aware operators](https://peps.python.org/pep-0505/)

[Null in Python: Understanding Python's NoneType Object](https://realpython.com/null-in-python/)

[Built-in constants: None](https://docs.python.org/3/library/constants.html#None)


### Gestion de errores recuperables

#### `C#`

[Best practices for exceptions](https://learn.microsoft.com/en-us/dotnet/standard/exceptions/best-practices-for-exceptions)

[Better error handling in C# with Result types](https://dev.to/ephilips/better-error-handling-in-c-with-result-types-4aan)

[Result Pattern in C#](https://medium.com/@walticotc/result-pattern-in-c-537bedda17a6)

[What is best practice for returning value or error message from method in c#?](https://stackoverflow.com/questions/29768502/what-is-best-practice-for-returning-value-or-error-message-from-method-in-c)

#### Java

[Java - How to represent the result of an operation](https://codereview.stackexchange.com/questions/20285/java-how-to-represent-the-result-of-an-operation)

[List of Java Exceptions](https://programming.guide/java/list-of-java-exceptions.html)

[Unchecked Exceptions — The Controversy](https://docs.oracle.com/javase/tutorial/essential/exceptions/runtime.html)

#### Python

[Best practice in python for return value on error vs. success](https://stackoverflow.com/questions/1630706/best-practice-in-python-for-return-value-on-error-vs-success)

[Mastering the Result Pattern in Software Development](https://www.codingexplorations.com/blog/mastering-the-result-pattern-in-software-development)


### Pattern matching

#### `C#`

[Pattern matching overview](https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/functional/pattern-matching)

#### Java

[Pattern Matching](https://docs.oracle.com/en/java/javase/21/language/pattern-matching.html)

[Pattern Matching for Switch](https://docs.oracle.com/en/java/javase/21/language/pattern-matching.html)

#### Python

[The match statement](https://docs.python.org/3/reference/compound_stmts.html#match)


### Trabajo sobre colecciones

[How To Start Using .map() .filter() and .reduce()](https://betterprogramming.pub/how-to-start-using-map-filter-and-reduce-e01edba0d81)

#### `C#`

[LINQ, Enumerable.Where Method](https://learn.microsoft.com/en-us/dotnet/api/system.linq.enumerable.where?view=net-8.0)

[LINQ, Enumerable.Select Method](https://learn.microsoft.com/en-us/dotnet/api/system.linq.enumerable.select?view=net-8.0)

[LINQ, Enumerable.Aggregate Method](https://learn.microsoft.com/en-us/dotnet/api/system.linq.enumerable.aggregate?view=net-8.0)

#### Java

[Package java.util.stream Description](https://docs.oracle.com/javase/8/docs/api/java/util/stream/package-summary.html#package.description)

[Processing Data in Memory Using the Stream API](https://dev.java/learn/api/streams/map-filter-reduce/)

#### Python

[Python's filter(): Extract Values From Iterables](https://realpython.com/python-filter-function/)

[Python's map(): Processing Iterables Without a Loop](https://realpython.com/python-map-function/)

[Python's reduce(): From Functional to Pythonic Style](https://realpython.com/python-reduce-function/)

