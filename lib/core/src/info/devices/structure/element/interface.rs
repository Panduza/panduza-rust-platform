use serde_json::json;

use super::StructuralElement;

use crate::Error;

///
///
///
pub struct ElementInterface {
    name: String,
    tags: Vec<String>,
    elements: Vec<StructuralElement>,
}

impl ElementInterface {
    pub fn new<N: Into<String>>(name: N, tags: Vec<String>) -> Self {
        Self {
            name: name.into(),
            tags,
            elements: Vec::new(),
        }
    }

    pub fn into_json_value(&self) -> serde_json::Value {
        //
        let mut children = serde_json::Map::new();
        for e in &self.elements {
            children.insert(e.name().clone(), e.into_json_value());
        }

        return json!({
            "tags": self.tags,
            "children": children
        });
    }

    pub fn is_element_exist(&self, layers: Vec<String>) -> Result<bool, Error> {
        // TODO Control layers == 0

        if layers.len() == 1 {
            let name = layers.get(0).ok_or(Error::Wtf)?;
            for element in &self.elements {
                if element.name() == name {
                    return Ok(true);
                }
            }
            return Ok(false);
        } else {
            let name = layers.get(0).ok_or(Error::Wtf)?;
            let sublayer = self.find_layer(&name);

            let mut new_la = layers;
            new_la.remove(0);
            return sublayer.is_element_exist(new_la);
        }
    }

    ///
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    ///
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn find_layer(&self, name: &str) -> &StructuralElement {
        self.elements
            .iter()
            .find(|element| element.name() == name)
            .unwrap_or_else(|| {
                panic!("Layer '{}' not found in device", name);
            })
    }

    pub fn find_layer_mut(&mut self, name: &str) -> &mut StructuralElement {
        self.elements
            .iter_mut()
            .find(|element| element.name() == name)
            .unwrap_or_else(|| {
                panic!("Layer '{}' not found in device", name);
            })
    }

    pub fn insert(&mut self, layers: Vec<String>, element: StructuralElement) -> Result<(), Error> {
        if layers.len() == 1 {
            // Insert HERE
            // new element name = layers.get(0)
            let layer_name = match layers.get(0) {
                Some(value) => {
                    self.elements.push(element);
                }
                None => {
                    // None
                    // TODO SO UGLY
                    return Err(Error::Generic("layer name not found 2".to_string()));
                    // cannot find the layer => error
                }
            };

            // insert here
        } else {
            // Insert inside the sub layer
            let layer_name = match layers.get(0) {
                Some(value) => Some(value.to_string()),
                None => {
                    None
                    // Err(Error::Generic("layer name not found".to_string()))
                    // cannot find the layer => error
                }
            };

            match layer_name {
                Some(n) => {
                    let mut new_layers = layers;
                    new_layers.remove(0);
                    let sublayer = self.find_layer_mut(&n);
                    sublayer.insert(new_layers, element)?;
                }
                None => todo!(),
            }
        }

        Ok(())
    }
}
