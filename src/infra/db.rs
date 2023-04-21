use diesel::{self, PgConnection};

use std::thread;
use std::time::Duration;

use super::config::AppConfig;

pub struct DB {
    pub conn: PgConnection,
}

pub fn new_connection(c: &AppConfig) -> DB {
    DB {
        conn: DB::establish_connection(
            &c.db.host,
            &c.db.port,
            &c.db.user,
            &c.db.password,
            &c.db.database,
        ),
    }
}