use crate::services::configuration_provider::ConfigurationProvider;
use once_cell::sync::OnceCell;
use rsfbclient::SimpleConnection;
use shaku::{Component, Interface};
use std::sync::Arc;
use std::{env, sync::Mutex};

pub static DATABASE_CONNECTION: OnceCell<Mutex<SimpleConnection>> = OnceCell::new();

fn get_db_conn(database_name: String) -> &'static Mutex<SimpleConnection> {
    DATABASE_CONNECTION.get_or_init(|| {
        let cwd = env::current_dir().unwrap();
        let db_path = cwd.join(database_name);
        let db_path_string = db_path.to_string_lossy().to_string();
        println!("database path: {}", db_path_string);

        let mut builder = rsfbclient::builder_native().with_dyn_link().with_embedded();

        let conn = if !db_path.exists() {
            builder
                .db_name(&db_path_string)
                .user("sysdba")
                .create_database()
                .expect("database is not exists, and fail to create one!!.")
        } else {
            builder
                .db_name(&db_path_string)
                .user("sysdba")
                .connect()
                .expect("fail to create connection")
        };

        Mutex::new(conn.into())
    })
}

pub trait ConnectionProvider: Interface {
    fn get_conn(&self) -> &'static Mutex<SimpleConnection>;
}

#[derive(Component)]
#[shaku(interface = ConnectionProvider)]
pub struct ConnectionProviderImpl {
    #[shaku(inject)]
    configuration_provider: Arc<dyn ConfigurationProvider>,
}

impl ConnectionProvider for ConnectionProviderImpl {
    fn get_conn(&self) -> &'static Mutex<SimpleConnection> {
        let config = self.configuration_provider.get_config();
        get_db_conn(config.database_name)
    }
}
