
use std::{fs, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

use tracing::{debug, info, trace};
use tracing_log::LogTracer;
use tracing_subscriber::{ Registry};
use tracing_subscriber::layer::SubscriberExt;
use rustapp::ThreadPool;

fn main() {
    LogTracer::init().expect("Unable to setup log tracer!");
    let fmt_layer_json = tracing_subscriber::fmt::layer().with_file(true).with_level(true).with_line_number(true).with_thread_ids(true).with_thread_names(true).compact();
    let subscriber = Registry::default()
        .with(fmt_layer_json);
    tracing::subscriber::set_global_default(subscriber).unwrap();
    info!("Started web app");

    let listener = TcpListener::bind("localhost:8080").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        info!("Connection established!: {}",_stream.local_addr().unwrap().to_string());
        // break
        // pool.execute(|| {
        //     handle_connection(_stream)
        // });
        handle_connection(_stream);
    }
    info!("Hello, world!");
    trace!("Commencing yak shaving");
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "src/index.html", ),
        "GET /sleep HTTP/1.1" => {
            debug!("sleeping for 5 minutes");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "src/index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "src/404.html")
    };
    debug!("{} {}",status_line,filename);
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );
    // let http_request:Vec<_> = buf_reader.lines().map(|read| read.unwrap()).take_while(|line|!line.is_empty()).collect();
    stream.write_all(response.as_bytes()).unwrap();
}