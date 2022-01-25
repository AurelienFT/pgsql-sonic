use tokio_postgres::{tls::MakeTlsConnect, tls::TlsConnect, Client, Config, Error, Socket};

pub struct Connection<T> {
    config: Config,
    tls_connector: T,
}

impl<T> Connection<T>
where
    T: MakeTlsConnect<Socket> + Clone + 'static + Sync + Send,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
{
    pub async fn new(config: Config, tls_connector: T) -> Result<(Connection<T>, Client), Error> {
        let (client, connection) = config.connect(tls_connector.clone()).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
            println!("Connection is closed");
        });
        Ok((
            Connection {
                config,
                tls_connector,
            },
            client,
        ))
    }
}
