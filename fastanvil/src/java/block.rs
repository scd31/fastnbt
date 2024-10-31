use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, PartialEq, Clone)]
pub enum BlockArchetype {
    Normal,
    Airy,
    Watery,
    Snowy,
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Block {
    pub name: String,

    #[serde(default)]
    pub properties: HashMap<String, String>,
}

impl Block {
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: HashMap::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn snowy(&self) -> bool {
        self.properties.get("snowy").map(String::as_str) == Some("true")
    }

    pub fn archetype(&self) -> BlockArchetype {
        if self.snowy() {
            BlockArchetype::Snowy
        } else if is_watery(&self.name) {
            BlockArchetype::Watery
        } else if is_airy(&self.name) {
            BlockArchetype::Airy
        } else {
            BlockArchetype::Normal
        }
    }

    /// A string of the format "id|prop1=val1,prop2=val2". The properties are
    /// ordered lexigraphically. This somewhat matches the way Minecraft stores
    /// variants in blockstates, but with the block ID/name prepended.
    pub fn encoded_description(&self) -> String {
        let mut props = self
            .properties
            .iter()
            .filter(|(k, _)| *k != "waterlogged") // TODO: Handle water logging. See note below
            .filter(|(k, _)| *k != "powered") // TODO: Handle power
            .collect::<Vec<_>>();

        // need to sort the properties for a consistent ID
        props.sort_unstable();

        let id = props
            .iter()
            .map(|(k, v)| format!("{k}={v}"))
            .collect::<Vec<_>>()
            .join(",");

        format!("{}|{}", self.name.clone(), id)
    }
}

/// Blocks that are considered as if they are water when determining colour.
fn is_watery(block: &str) -> bool {
    matches!(
        block,
        "minecraft:water"
            | "minecraft:bubble_column"
            | "minecraft:kelp"
            | "minecraft:kelp_plant"
            | "minecraft:sea_grass"
            | "minecraft:tall_seagrass"
    )
}

/// Blocks that are considered as if they are air when determining colour.
fn is_airy(block: &str) -> bool {
    matches!(block, "minecraft:air" | "minecraft:cave_air")
}
