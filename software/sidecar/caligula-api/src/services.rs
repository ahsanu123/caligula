use crate::services::configuration_provider::ConfigurationProvider;
use crate::services::configuration_provider::ConfigurationProviderFactory;
use crate::services::configuration_provider::ConfigurationProviderImpl;
use crate::services::connection_provider::ConnectionProviderImpl;
use shaku::HasComponent;
use shaku::HasProvider;
use shaku::module;

pub mod configuration_provider;
pub mod connection_provider;

module! {
    pub ServicesCollection {
        components  = [
            ConfigurationProviderImpl,
            ConnectionProviderImpl
        ],
        providers = [
            // NOTE:
            // for now this here is just sample for using provide in shaku
            ConfigurationProviderFactory
        ]
    }
}

#[cfg(test)]
mod test_services {

    use rsfbclient::Queryable;
    use rsfbclient::Row;

    use crate::services::connection_provider::ConnectionProvider;

    use super::*;

    #[test]
    fn test_configuration_provider_factory() {
        let module = ServicesCollection::builder().build();

        let configuration_provider = module.provide().unwrap();
        let config = configuration_provider.get_config();

        println!("configuration by factory: {:#?}", config.clone());
    }

    #[test]
    fn test_configuration_provider_component() {
        let module = ServicesCollection::builder().build();

        let configuration_provider: &dyn ConfigurationProvider = module.resolve_ref();

        let config = configuration_provider.get_config();

        println!("configuration by component: {:#?}", config.clone());
    }

    #[test]
    fn test_database_get_connection() {
        let module = ServicesCollection::builder().build();

        let connection_provider: &dyn ConnectionProvider = module.resolve_ref();

        let mut conn = connection_provider
            .get_conn()
            .lock()
            .expect("fail to lock connection mutex");

        let rows: Vec<Row> = conn
            .query("SELECT 1 FROM RDB$PROCEDURES;", ())
            .expect("fail to do query");

        for row in rows {
            println!("------------------------------------");
            for col in row.cols {
                println!("{}: {:?}", col.name, col.value);
            }
        }
    }
}
