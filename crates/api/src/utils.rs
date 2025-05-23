use anyhow::Result;
use rusqlite::Connection;

pub fn print_db(db: &Connection) -> Result<()> {
    let tables: Vec<_> = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")?
        .query_map([], |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })?
        .into_iter()
        .collect();

    for table in tables {
        let table = table?;
        println!("table: {table}");
        let mut stmt = db.prepare(&format!("SELECT * FROM {table}"))?;

        let col_names = stmt.column_names();

        let col_count = col_names.len();
        for name in col_names {
            print!("{name} ");
        }
        println!("");

        let mut rows = stmt.query([])?;
        while let Some(next) = rows.next()? {
            for idx in 0..col_count {
                let s = match next.get_ref(idx)? {
                    rusqlite::types::ValueRef::Null => "NULL".to_string(),
                    rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                    rusqlite::types::ValueRef::Real(f) => f.to_string(),
                    rusqlite::types::ValueRef::Text(t) => String::from_utf8_lossy(t).to_string(),
                    rusqlite::types::ValueRef::Blob(b) => format!("{:?}", b),
                };
                print!("{s} ");
            }
            println!();
        }
    }

    return Ok(());
}
