use std::{
    error,
    io::{self, prelude::*},
    net, process, str,
};
extern crate ctrlc;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut stream = net::TcpStream::connect("127.0.0.1:5000")?;
    let s = stream.try_clone()?;
    ctrlc::set_handler(move || {
        s.shutdown(net::Shutdown::Both).unwrap();
        process::exit(0);
    })?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        let mut reader = io::BufReader::new(&stream);
        reader.fill_buf()?;
        print!("{}", str::from_utf8(reader.buffer())?);
    }
}
