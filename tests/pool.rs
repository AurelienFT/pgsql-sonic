mod tests {
    use pgsql_sonic::pool::Pool;
    use tokio_postgres::NoTls;
    use tokio::join;
    #[tokio::test]
    async fn basic_pool() {
        let mut pool = Pool::new();
        let client = pool
            .get("host=localhost user=postgres".parse().unwrap(), NoTls)
            .await
            .unwrap();
        join!(client.simple_query("SELECT 1"), client.simple_query("SELECT 1"));
    }
}
