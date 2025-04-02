use crate::server::start_server;

mod leibniz_adder;
mod server;
mod errors;
/*
TODO:
- Clean code
    - Differ request by method type (GET, POST, etc.)
- TP2 Requirements
- Start TP3
 */

fn main() {
    start_server();
}
