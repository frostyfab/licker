use machineid_rs::{Encryption, HWIDComponent, IdBuilder};
use std::error::Error;
use std::result::Result;

/// Generates a hashed unique identifier for the system using the machineid_rs library.
pub fn id() -> Result<String, Box<dyn Error + 'static>> {
    let mut builder = IdBuilder::new(Encryption::SHA256);

    builder
        .add_component(HWIDComponent::SystemID)
        .add_component(HWIDComponent::CPUID)
        .add_component(HWIDComponent::DriveSerial);

    let id = builder.build("licker")?;

    Ok(id)
}
