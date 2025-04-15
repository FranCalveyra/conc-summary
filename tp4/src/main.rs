/*
Requerimientos:
- Subir archivos
- Obtener stats (se calculan al momento de subirlo, el file no se vuelve a usar)
    - Esas stats refieren a la cantidad de excepciones que se encuentran
- Control de concurrencia: No se deben procesar m´as de 4 archivos
    en simult´aneo. En el caso de que no se pueda procesar un archivo, se
    debe devolver un error 429 Too Many Requests.
- Acceso concurrente a datos compartidos:
  Este servidor ser´a mayormente usado para consultas de las estad´ısticas. Considerar esto en
  el dise˜no final para maximizar la concurrencia de lectura.
 */

/*
  => No persistimos los archivos, matcheamos por nombre y punto
*/
mod connection_handler;
mod errors;
mod log_analyzer;
mod server;

use std::sync::Arc;
use thread_pool::thread_pool::get_system_thread_amount;

fn main() {
    let mut log_server = Arc::new(server::Server::new());
    log_server.start(get_system_thread_amount())
}
