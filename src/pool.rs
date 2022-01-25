use crate::connection::Connection;
use tokio_postgres::{
    tls::MakeTlsConnect, tls::TlsConnect, Client, Config, Error, Socket,
};

pub struct Pool<T> {
    number_active_connections: usize,
    connections: Vec<Connection<T>>,
}

impl<T> Pool<T>
where
    T: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub fn new() -> Pool<T> {
        Pool {
            number_active_connections: 0,
            connections: Vec::new(),
        }
    }

    pub async fn get(&mut self, config: Config, tls_connector: T) -> Result<Client, Error> {
        let (internal_connection, client) = Connection::new(config.clone(), tls_connector).await?;
        self.number_active_connections += 1;
        self.connections.push(internal_connection);
        Ok(client)
    }

    pub fn get_number_active_connections(&self) -> usize {
        self.number_active_connections
    }
}
