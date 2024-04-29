use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req = buf_reader.lines().next().unwrap().unwrap();
    let req_split: Vec<&str> = req.split(' ').collect();
    let path = req_split[1];

    let cwd = env!("PWD");
    let file_path = format!("{cwd}{path}");
    println!("{}", file_path);

    let status;
    let content = match fs::read_to_string(file_path) {
        Ok(f) => {
            status = "200";
            f
        }
        Err(_) => {
            status = "404";
            String::from("<h1>404 Not found</h1>")
        }
    };

    println!("{}", content);
    let headers = format!("HTTP/1.1 {status}\r\nContent-type: text/html;");
    let res = format!("{headers}\r\n\r\n{content}\r\n\r\n");

    stream.write_all(res.as_bytes()).unwrap()
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4567").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("{:?}", stream);
        handle_client(stream);
    }
}
