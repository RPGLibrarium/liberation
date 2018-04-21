use mysql;
use dmos;

type Error = mysql::error::Error;

pub static INIT_DB_STRUCTURE: &str = include_str!("../res/init-db-structure.sql");

pub struct Database {
    pool: mysql::Pool
}

impl Database {
    pub fn new(url:String) -> Result<Database, Error> {
        let pool = mysql::Pool::new(url)?;

        let mut conn = pool.get_conn()?;
        conn.query(INIT_DB_STRUCTURE)?;

        return Ok(Database{
            pool: pool,
        })
    }

    pub fn insert_rpg_system(&self, name: &String) -> Result<dmos::RpgSystem, Error> {
        return self.pool.prep_exec("insert into rpg_systems (name) values (:name)",
            params!{
                "name" => name,
            }).map(|result| {
                dmos::RpgSystem {
                    id: result.last_insert_id(),
                    name: name.clone(),
                }
            })
    }

    pub fn get_rpg_systems(&self) -> Result<Vec<dmos::RpgSystem>, Error> {
        return self.pool.prep_exec("select * from rpg_systems;",())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let (id, name) = mysql::from_row(row);
                dmos::RpgSystem {
                    id: id,
                    name: name,
                }
            }).collect()
        });
    }


}
