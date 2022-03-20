use std::time::Duration;
use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.ejecuta(|| {
            gestiona_conexion(stream)
        });
    }
}

fn gestiona_conexion(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let duerme = b"GET /duerme HTTP/1.1\r\n";

    let (status, fichero) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "hello.html")
        } else if buffer.starts_with(duerme) {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

        let contenido = fs::read_to_string(fichero).unwrap();

        let respuesta = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status,
            contenido.len(),
            contenido
            );

        stream.write(respuesta.as_bytes()).unwrap();
        stream.flush().unwrap();
}
