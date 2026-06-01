use crate::models::configuration::Configuration;
use config::Config;
use shaku::{Component, Interface, Module, Provider};
use std::error::Error;

/// Default for Configuration
impl Default for Configuration {
    fn default() -> Self {
        ConfigurationProviderImpl::load_configuration()
    }
}

pub trait ConfigurationProvider: Interface {
    fn get_config(&self) -> Configuration;
}

/// shaku Component
#[derive(Component)]
#[shaku(interface = ConfigurationProvider)]
pub struct ConfigurationProviderImpl {
    #[shaku(default)]
    configuration: Configuration,
}

impl ConfigurationProviderImpl {
    pub fn load_configuration() -> Configuration {
        let configuration = Config::builder()
            .add_source(config::File::with_name("configuration.json"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        configuration.try_deserialize::<Configuration>().unwrap()
    }
}

impl ConfigurationProvider for ConfigurationProviderImpl {
    fn get_config(&self) -> Configuration {
        self.configuration.clone()
    }
}

/// ConfigurationProviderFactory
/// this will run for every call
///
/// let module = ServicesCollection::builder().build();
/// let configuration_provider = module.provide().unwrap();
/// let config = configuration_provider.get_config();
pub struct ConfigurationProviderFactory;

impl<M> Provider<M> for ConfigurationProviderFactory
where
    M: Module,
{
    type Interface = dyn ConfigurationProvider + 'static;

    fn provide(_module: &M) -> Result<Box<Self::Interface>, Box<dyn Error>> {
        Ok(Box::new(Self::build_configuration_provider()))
    }
}

impl ConfigurationProviderFactory {
    pub fn build_configuration_provider() -> ConfigurationProviderImpl {
        let configuration = Config::builder()
            .add_source(config::File::with_name("configuration.json"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        let configuration = configuration.try_deserialize::<Configuration>().unwrap();

        ConfigurationProviderImpl { configuration }
    }
}

#[cfg(test)]
mod test_configuration_provider {
    use super::*;

    #[test]
    fn test_config_from_json() {
        let settings = Config::builder()
            .add_source(config::File::with_name("configuration.json"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .unwrap();

        // Print out our settings (as a HashMap)
        println!(
            "{:#?}",
            settings.try_deserialize::<Configuration>().unwrap()
        );
    }
}
