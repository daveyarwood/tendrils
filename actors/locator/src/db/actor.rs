use rusqlite;
use time::Timespec;

#[derive(Clone, Debug)]
pub struct Actor {
    pub id: String,
    pub location: String,
    pub last_seen: Timespec,
    pub online: bool
}

pub fn locate_actor(conn: &rusqlite::Connection, actor_id: String)
    -> Result<Option<Actor>, rusqlite::Error> {
    let mut stmt = try!(conn.prepare(
        "SELECT id, location, last_seen, online
           FROM locator_actor
          WHERE id = :actor_id"));

    let rows = try!(stmt.query_map(&[&actor_id], |row| {
        Actor {
            id: row.get(0),
            location: row.get(1),
            last_seen: row.get(2),
            online: row.get(3)
        }
    }));

    let mut actors = Vec::<Actor>::new();
    for row in rows {
        let row = try!(row);
        actors.push(row);
    }

    Ok(actors.first().map(|a| a.clone()))
}
