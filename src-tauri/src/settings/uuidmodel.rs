use machineid_rs::{ Encryption, HWIDComponent, IdBuilder };
use anyhow::Result;

#[derive(Debug, Default)]
pub struct UniqueId {
    pub id: String,
}

impl UniqueId {
    pub fn default() -> Result<Self> {
        const KEY: &str = "TODO:CHANGE_THIS_KEY";
        let mut builder = IdBuilder::new(Encryption::SHA256);
        builder
            .add_component(HWIDComponent::SystemID)
            .add_component(HWIDComponent::MachineName)
            .add_component(HWIDComponent::Username);

        let uuid = builder.build(KEY)?;

        Ok(UniqueId {
            id: uuid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let generated = UniqueId::default().unwrap();
        assert_eq!(generated.id.len(), 64);
    }
}
