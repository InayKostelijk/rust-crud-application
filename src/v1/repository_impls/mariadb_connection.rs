use sqlx::MySqlPool;

pub struct MariaDbConnection{
    pool: MySqlPool,
}

impl MariaDbConnection {
    pub fn new(pool: MySqlPool) -> Self {
        Self{
            pool
        }
    }
}