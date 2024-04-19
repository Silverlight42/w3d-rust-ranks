use std::io::{self, BufReader, BufRead, Read, ErrorKind};
use std::net::TcpStream;

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("./w3d.db")?;

    conn.execute(
        "create table if not exists players (
            id integer primary key,
            name text not null unique
        )", (),
    )?;

    match connect_to_server("127.0.0.1", 7846) {
        Ok(mut stream) => {
            println!("Connected!");
            // read_from_stream(&mut stream);
            read_from_stream_continously(&mut stream);
            // print_stream_to_console(&mut stream);
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }


    Ok(())
}

fn read_from_stream_continously(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);

    loop {
        let mut buffer = Vec::new();
        match reader.read_until(0x00, &mut buffer) {
            Ok(n) => {
                if n == 0 {
                    // The stream has been closed
                    break;
                }
                &buffer.pop();
                println!("{:?}", String::from_utf8_lossy(&buffer).trim_matches('\"'));
            }
            Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                // A signal interrupted the read operation, retry
                continue;
            }
            Err(e) => {
                // An error occurred
                return Err(e);
            }
        }
    }

    Ok(())
}

fn read_from_stream(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

fn print_stream_to_console(stream: &mut TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);

    let mut buffer = Vec::new();
    reader.read_until(0x00, &mut buffer)?;
    println!("Received: {:?}", String::from_utf8_lossy(&buffer));

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        let splitted = line.split('\0');
        for split in splitted {
            println!("{}", split);
        }
    }

    Ok(())
}

fn connect_to_server(ip: &str, port: u16) -> io::Result<TcpStream> {
    let address = format!("{}:{}", ip, port);
    let stream = TcpStream::connect(&address)?;
    Ok(stream)
}