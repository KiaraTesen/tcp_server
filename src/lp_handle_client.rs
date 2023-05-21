use std::net::{TcpStream};
use std::io::{BufRead, BufReader, Error, Write};
use std::{str};


#[derive(Serialize, Deserialize, Debug)]
struct MessageSerialized {
    value: f64,
    best_vector: Vec<f64>
}

static mut ID_EXPERIMENTO: f64= 0.0 ;



pub fn handle_client(stream: TcpStream, subnet : String) -> Result<(), Error> {
    
    let ip_entrante = stream.peer_addr()?.to_string(); 

    let prueba = ip_entrante.starts_with(&subnet);

    if prueba{
        
        let mut data = Vec::new();
        let mut stream = BufReader::new(stream);
        
        
        loop {
            data.clear();
    
            let bytes_read = stream.read_until(b'\n', &mut data)?;
            if bytes_read == 0 {
                return Ok(());
            }
            println!("\nIncoming connection from: {}", ip_entrante);
            println!("ID experimento : {}", unsafe { ID_EXPERIMENTO});
            println!("{:?\n}", chrono::offset::Local::now());
    
            unsafe { ID_EXPERIMENTO += 1.0};
    
            unsafe { write!(stream.get_mut(), "{}", &ID_EXPERIMENTO)?};
            write!(stream.get_mut(), "{}", "\n")?;
        }
    } else{
        return Ok(());
    };
}
