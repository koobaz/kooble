use rusqlite::Connection;

pub fn insert_data(db: String , content: String) -> Result<(), Box<dyn std::error::Error>>{
    let conn = Connection::open(db)?;
    conn.execute(
        "INSERT INTO urls (url) VALUES (?1)",
        [content]
    )?;
    Ok(())
}
