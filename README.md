# Simulator Center
Aqui se encuentran todos los proyectos relacionados con el trabajo de titulo incluyendo el codigo de los experimentos que realizo con los borradores del mismo.

Para clonar el repositorio utilice `git clone --recurse-submodules https://github.com/PatatasDelPapa/simulator_center.git` esto descargara este repositorio con todos los submodulos. Para compilar los proyectos en este repositorio es necesario instalar [Rust](https://www.rust-lang.org/), preferentemente utilizando [rustup](https://rustup.rs/).

## Estructura
Este repositorio es un [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) donde cada submodulo es un miembro. Es posible ejecutar comandos de cargo accediendo a la carpeta del proyecto y despues ejecutar el comando
Ejemplo:
```
cd simulator
cargo run --example search_1
```
O se puede usar la configuración del workspace para hacerlo desde carpeta de este repositorio (es decir sin tener que cambiar de directorio)
```
cargo run -p simulator --example search_1
```
A continuación se muestra una explicación breve de cada submodulo.
### Documentacion
Borradores del documento de titulo que voy desarrollando, no incluye un proyecto compilable.
### Simulator
Codigo de la libreria que se esta desarrollando con ejemplos y benchmark incluido
### Desim Benchmark
Codigo para medir la performance de la libreria desim con un modelo simple
### Desim mpmc
Codigo para medir la performance de la libreria desim con un modelo un poco mas complejo
### Simrs Benchmark
Codigo para medir la performance de la libreria simrs con un modelo simple
### Simulator Benchmark
Codigo para medir la performance de la libreria simulator (la que estoy desarrollando) con un modelo simple de forma externa (como lo haria un usuario de la libreria)
