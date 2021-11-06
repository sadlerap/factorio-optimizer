use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A machine.  It produces materials using a recipe.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Machine {
    name: String,
    production_rate: f64,
}

impl Machine {
    pub fn new(name: String, production_rate: f64) -> Self {
        Self {
            name,
            production_rate,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn production_rate(&self) -> f64 {
        self.production_rate
    }
}

impl core::hash::Hash for Machine {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Machine {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Machine {}

#[derive(Hash, PartialEq, Eq, Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    name: String,
}

impl Product {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Recipe {
    /// Name of the recipe.
    name: String,
    /// Amount of time to produce in seconds.
    production_time: f64,
    /// How much of a product is used/produced in this recipe
    usage: HashMap<Product, f64>,
    /// How much of a product is used/produced in this recipe
    production: HashMap<Product, f64>,
}

impl Recipe {
    pub fn new(
        name: String,
        production_time: f64,
        usage: HashMap<Product, f64>,
        production: HashMap<Product, f64>,
    ) -> Self {
        Self {
            name,
            production_time,
            usage,
            production,
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn production_time(&self) -> f64 {
        self.production_time
    }

    pub fn usage_of(&self, product: &Product) -> Option<f64> {
        self.usage.get(product).copied()
    }

    pub fn production_of(&self, product: &Product) -> Option<f64> {
        self.production.get(product).copied()
    }
}

impl core::hash::Hash for Recipe {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Recipe {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Recipe {}
