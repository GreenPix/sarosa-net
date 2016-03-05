use std::net::{TcpStream,SocketAddr,ToSocketAddrs};
use std::io::{self,BufReader,BufWriter,ErrorKind,Error,Write};
use std::fmt;
use std::error;

use lycan_serialize::Error as LycanError;
use lycan_serialize::ErrorKind as LycanErrorKind;
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

#[derive(Debug,Clone,Copy)]
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
        let size = match self.socket.read_u64::<LittleEndian>() {
            Err(err) => {
                error!("Network error: {}", err);
                return Err(NetworkError::DisconnectedFromServer);
            }
            Ok(size) => size,
        };
        Notification::deserialize(&mut self.socket,size).map_err(|err| {
            error!("Network error: {}", err);
            err.into()
        })
    }
}

pub fn connect(settings: &NetworkSettings)
-> Result<(NetworkReader, NetworkWriter),NetworkError> {
    let tokens = messages::forge_authentication_tokens();
    for (id,token) in tokens {
        trace!("Connecting to server");
        let mut stream = try!(TcpStream::connect(settings.server_addr));
        trace!("Connected to server");
        let command = GameCommand::Authenticate(id, token);
        try!(command.serialize(&mut stream));
        let size = match stream.read_u64::<LittleEndian>() {
            Err(err) => {
                error!("Network error: {}", err);
                return Err(NetworkError::DisconnectedFromServer);
            }
            Ok(size) => size,
        };
        let response = try!(Notification::deserialize(&mut stream,size));
        match response {
            Notification::Response{code: ErrorCode::Success} => {
                trace!("Success, connected to server");
                let reader = NetworkReader::new(try!(stream.try_clone()));
                let writer = NetworkWriter::new(stream);
                return Ok((reader,writer));
            }
            Notification::Response{code: ErrorCode::Error} => {
                trace!("Failed to authenticate with token {:?}", token);
            }
            other => {
                error!("Unexpected notification {:?} (expected a response)", other);
            }
        }
    }
    error!("No valid token, impossible to authenticate");
    Err(NetworkError::DisconnectedFromServer)
}

impl From<LycanError> for NetworkError {
    fn from(err: LycanError) -> NetworkError {
        match err.kind {
            LycanErrorKind::Disconnected => {
                NetworkError::DisconnectedFromServer
            }
            _ => NetworkError::DeserializationError,
        }
    }
}

impl From<io::Error> for NetworkError {
    fn from(err: io::Error) -> NetworkError {
        match err.kind() {
            io::ErrorKind::BrokenPipe => NetworkError::DisconnectedFromServer,
            io::ErrorKind::ConnectionAborted => NetworkError::DisconnectedFromServer,
            _ => NetworkError::UnknownError,
        }
    }
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for NetworkError {
    fn description(&self) -> &str {
        match *self {
            NetworkError::DisconnectedFromServer => "Disconnected from server",
            NetworkError::DeserializationError   => "Deserialization error",
            NetworkError::UnknownError           => "Unknown error",
        }
    }
}
