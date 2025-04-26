use std::marker::PhantomData;

use bevy::{
    asset::{Asset, AssetLoader, LoadContext, io::Reader},
    reflect::TypePath,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct YamlLoaderSettings;

#[derive(Default)]
pub struct GenericYamlAssetLoader<T> {
    marker: PhantomData<T>,
}

impl<T> AssetLoader for GenericYamlAssetLoader<T>
where
    T: DeserializeOwned + TypePath + Asset + Send + Sync + 'static,
{
    type Asset = T;
    type Settings = YamlLoaderSettings;
    type Error = anyhow::Error;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;

        let yaml_str = std::str::from_utf8(&bytes)?;
        let custom_asset: T = serde_yaml::from_str(&yaml_str)?;
        Ok(custom_asset)
    }

    fn extensions(&self) -> &[&str] {
        &["yaml", "yml"]
    }
}
