use std::io::{self, Write, Read};
use std::net::TcpStream;
use std::thread;

fn receive_messages(mut stream: TcpStream)
{
    let mut buffer = [0u8; 4096];
    let mut previous_message_len = 0;

    loop 
    {
        match stream.read(&mut buffer)
        {
            Ok(n) if n == 0 => {
                println!("Server closed the connection");
                break;
            }

            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[0..n]);
                print!("\r{}{}", message, " ".repeat((message.len() as i32 - previous_message_len as i32).abs() as usize));
                previous_message_len = message.len();
            }

            Err(err) => {
                eprintln!("Error reading from the server: {}", err);
                break;
            }
        }    
    }
}

fn main() -> io::Result<()>
{
    let mut input = String::new();

    print!("Enter IP: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    
    let server_ip = input.trim();
    let server_address = format!("{}:8080", server_ip);
    
    match TcpStream::connect(server_address)
    {
        Ok(mut stream) => {
            println!("Connected");

            // spawn thread to receive messages from server
            let server_stream = stream.try_clone().expect("Failed to clone server stream");
            thread::spawn(move || {
                receive_messages(server_stream);
            });

            loop 
            {
                input.clear();
                print!(">");
                io::stdout().flush()?;
                io::stdin().read_line(&mut input)?;

                // send
                stream.write(input.as_bytes())?;
            }
        }

        Err(err) => {
            eprintln!("Connection failed: {}", err);
        }
    }

    Ok(())
}