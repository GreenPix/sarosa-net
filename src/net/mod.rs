use std::net::{TcpStream,SocketAddr,ToSocketAddrs};
use std::io::{self,BufReader,BufWriter,ErrorKind,Error,Write};

use capnp::Error as CapnpError;

use messages::{self,TargettedOrder,Notification};

#[derive(Debug,Clone,Copy)]
pub struct NetworkSettings {
    server_addr: SocketAddr,
    // TODO
    // AuthenticationToken
}

impl NetworkSettings {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Result<NetworkSettings, Error>{
        let server_addr = match try!(addr.to_socket_addrs()).next() {
            Some(address) => address,
            None => {
                return Err(Error::new(ErrorKind::InvalidInput, "Could not resolve address"));
            }
        };

        Ok(NetworkSettings {
            server_addr: server_addr,
        })
    }
}

pub struct NetworkReader {
    socket: BufReader<TcpStream>,
}

pub struct NetworkWriter {
    socket: BufWriter<TcpStream>,
}

impl NetworkWriter {
    fn new(socket: TcpStream) -> NetworkWriter {
        NetworkWriter {
            socket: BufWriter::new(socket),
        }
    }

    pub fn write(&mut self, order: &TargettedOrder) -> Result<(), Error> {
        messages::serialize(&mut self.socket, order)
    }

    pub fn flush(&mut self) -> Result<(),Error> {
        self.socket.flush()
    }
}

pub enum NetworkError {
    DisconnectedFromServer,
    DeserializationError,
    UnknownError,
}

impl NetworkReader {
    fn new(socket: TcpStream) -> NetworkReader {
        NetworkReader {
            socket: BufReader::new(socket),
        }
    }

    pub fn read(&mut self) -> Result<Notification, NetworkError> {
        messages::deserialize(&mut self.socket).map_err(|err| {
            match err {
                CapnpError::Decode { .. } => NetworkError::DeserializationError,
                CapnpError::Io(e) => {
                    match e.kind() {
                        io::ErrorKind::BrokenPipe => NetworkError::DisconnectedFromServer,
                        io::ErrorKind::ConnectionAborted => NetworkError::DisconnectedFromServer,
                        _ => NetworkError::UnknownError,
                    }
                }
            }
        })
    }
}

pub fn connect(settings: &NetworkSettings)
-> Result<(NetworkReader, NetworkWriter),CapnpError> {
    let tokens = messages::forge_authentication_tokens();
    for token in tokens {
        let mut stream = try!(TcpStream::connect(settings.server_addr));
        try!(messages::send_authentication_token(&mut stream, &token));
        let response = try!(messages::deserialize_error_code(&mut stream));
        if response == 0 {
            let reader = NetworkReader::new(try!(stream.try_clone()));
            let writer = NetworkWriter::new(stream);
            return Ok((reader,writer));
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "Invalid authentication tokens").into())
}
