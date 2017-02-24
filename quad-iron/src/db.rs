use diesel::sqlite::SqliteConnection;
use iron::typemap::Key;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn init_pool(database_file: &str) -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<SqliteConnection>::new(database_file);
    r2d2::Pool::new(config, manager).expect("db pool")
}

pub struct DB;

impl Key for DB {
    type Value = Pool;
}
