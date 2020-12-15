# lexer y parser

## Proyecto

Para utilizar lalrpop, vamos a crear un nuevo proyecto en rust.

```bash
cargo new incr
```

Hay que hacer unos cambios al `Cargo.toml`

```toml
{{ #include ../../../Cargo.toml }}
```

La linea de `build` indica que queremos que cargo corra codigo adicional cuando
hagamos `cargo build`. El archivo `build.rs` contiene el codigo a correr (ver abajo).
El archivo debe estar ubicado junto con el `Cargo.toml`. Luego en `build-depencencies`,
indicamos que queremos lalrpop, y que lalrpop genere el lexer. Por ultimo, en
`dependencies` traemos unas utilidades.

El archivo `build.rs` deben crearlo con el siguente contenido:
```rust
{{#include ../../../build.rs}}
```

## lexer y parser

El lexer y parser lo especificaremos en el archivo `src/nums.lalrpop`:
```rust
{{#include ../../nums.lalrpop}}
```
Este archivo importa la funciones de `FromStr` que utilizamos para convertir
`String` a enteros. Luego declara una gramatica, que tiene un elemento publico
`Num`. Si encontramos un `String` que consiste de solo digitos `r"[0-9]+"`
entonces convertimos esa secuencia de digitos a un `i64`. La expression regular
`r"[0-9]+"` es utilizada para crear el lexer, y lo que va a la derecha de `=>`
es código de rust que corre el parser cuando reconoce el lexema de la izquierda.

## main

Para terminar el compilador, vamos a crear un `src/main.rs` que llame el parser
y genere el ensamblador.
```rust
{{#include main.rs}}
```
## Probando el compilador

Podemos probar el compilador corriendolo con cargo:
```bash
cargo run 123.jn
```
