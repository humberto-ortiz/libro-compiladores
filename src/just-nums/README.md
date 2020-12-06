# Números

Vamos a construir un pequeño compilador que puede entender enteros.

No es un lenguaje muy interesante, pero el compilador tiene todos los razgos
principales de un compilador de verdad, y podemos ir construyendo sobre el.

Lo primero es construir un proyecto de rust para nuestro compilador. Yo lo voy a llamar `just-numbers`, para recordarme que solo puede compilar numeros.

Ve a un directorio donde quieras almacenar el proyecto y corre:
```
cargo new just-numbers
```

esto crea un nuevo directorio `just-numbers` con varios archivos, incluyendo
`Cargo.toml` y `src/main.rs`. Cambiense a este nuevo directorio en la consola.


```
cargo build
cargo run 123.jn > 123.s
nasm -f elf64 -o 123.o 123.s
gcc -g -o 123 main.c 123.o
```
