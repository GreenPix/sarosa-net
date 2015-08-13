use std::net::{TcpStream,SocketAddr,ToSocketAddrs};
use std::io::{self,BufReader,BufWriter,ErrorKind,Error,Write};

use lycan_serialize::Error as CapnpError;
use byteorder::{ReadBytesExt,LittleEndian};

use messages::{self,EntityOrder,Notification,ErrorCode,GameCommand};

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

    pub fn write(&mut self, order: &EntityOrder) -> Result<(),Error> {
        order.serialize(&mut self.socket)
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
        let _size_notification = match self.socket.read_u64::<LittleEndian>() {
            Err(err) => {
                error!("Network error: {}", err);
                return Err(NetworkError::DisconnectedFromServer);
            }
            Ok(size) => size,
        };
        Notification::deserialize(&mut self.socket).map_err(|err| {
            error!("Network error: {}", err);
            match err {
                CapnpError::Io(e) => {
                    match e.kind() {
                        io::ErrorKind::BrokenPipe => NetworkError::DisconnectedFromServer,
                        io::ErrorKind::ConnectionAborted => NetworkError::DisconnectedFromServer,
                        _ => NetworkError::UnknownError,
                    }
                }
                _ => NetworkError::DeserializationError,
            }
        })
    }
}

pub fn connect(settings: &NetworkSettings)
-> Result<(NetworkReader, NetworkWriter),CapnpError> {
    let tokens = messages::forge_authentication_tokens();
    for token in tokens {
        let mut stream = try!(TcpStream::connect(settings.server_addr));
        let command = GameCommand::Authenticate(token);
        try!(command.serialize(&mut stream));
        let response = try!(ErrorCode::deserialize(&mut stream));
        match response {
            ErrorCode::Success => {
                let reader = NetworkReader::new(try!(stream.try_clone()));
                let writer = NetworkWriter::new(stream);
                return Ok((reader,writer));
            }
            ErrorCode::Error => {}
        }
    }
    Err(Error::new(ErrorKind::InvalidInput, "Invalid authentication tokens").into())
}
