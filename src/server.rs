use std::{net::{TcpListener, TcpStream}, io::Read};

use crate::http::Request;

pub struct Server {
    addr: String
}
    
impl Server {
    pub fn new(addr: String) -> Server {
        return Server{ addr };
    }
    
    pub fn run(self){
       println!("Listening on {}", self.addr);

       let listener = TcpListener::bind(&self.addr).unwrap();

       loop {
           let res = listener.accept();

           match res {
                Ok((mut stream, addr))=> {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_)=>{
                            let data_recived = String::from_utf8_lossy(&buffer);
                            println!("Recived data {}", data_recived);
                            match Request::try_from(&buffer[..]) {
                                Ok(req)=> {
                                    println!("Request = {:?}", req)
                                },
                                Err(_) =>{} 
                            }
                        },
                        Err(e)=> print!("Falied to read connection {}", e),
                    }

                },
                Err(e)=> print!("Error ocuuured {}", e)
           }
       }
    }
}