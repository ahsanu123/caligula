use crate::services::configuration_provider::ConfigurationProvider;
use crate::services::configuration_provider::ConfigurationProviderFactory;
use crate::services::configuration_provider::ConfigurationProviderImpl;
use shaku::HasComponent;
use shaku::HasProvider;
use shaku::module;

pub mod configuration_provider;

module! {
    ServicesCollection {
        components  = [ConfigurationProviderImpl],
        providers = [ConfigurationProviderFactory]
    }
}

#[cfg(test)]
mod test_services {

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
}
