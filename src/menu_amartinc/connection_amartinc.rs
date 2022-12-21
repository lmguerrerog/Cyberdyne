pub fn getConnection() -> sqlite::Connection {
    let connection = sqlite::open(":memory:").unwrap();
    connection
}