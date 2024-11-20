use super::{
    structure::{
        attribute::AttributElement,
        instance::{Alert, InstanceElement},
        Structure,
    },
    Topic,
};
use crate::underscore_device::structure::class::ClassElement;
use panduza_platform_core::{
    device::State, AlertNotification, Error, StateNotification, StructuralNotification,
};
use std::sync::Arc;
use tokio::sync::Notify;

pub struct InfoPackInner {
    ///
    ///
    ///
    structure: Structure,

    ///
    /// Notified when a device status change
    ///
    instance_status_change_notifier: Arc<Notify>,

    ///
    ///
    ///
    instance_structure_change_notifier: Arc<Notify>,
}

impl InfoPackInner {
    ///
    ///
    pub fn new() -> InfoPackInner {
        InfoPackInner {
            structure: Structure::default(),
            instance_status_change_notifier: Arc::new(Notify::new()),
            instance_structure_change_notifier: Arc::new(Notify::new()),
        }
    }

    ///
    /// Create a new instance if this instance does not already exist
    ///
    pub fn create_instance_if_not_exists(&mut self, instance_name: &String) {
        if !self.structure.contains_instance(&instance_name) {
            self.structure
                .insert_instance(instance_name.clone(), InstanceElement::default());
        }
    }

    ///
    ///
    ///
    pub fn process_state_changed(&mut self, n: &StateNotification) {
        let topic = Topic::from_string(n.topic.clone());
        // println!("{:?}", p.device);

        let instance_name = &topic.instance;

        //
        // Create the instance if not already created
        self.create_instance_if_not_exists(instance_name);

        //
        // Instance MUST now exist
        let instance = self
            .structure
            .get_mut_instance(instance_name)
            .ok_or(Error::Wtf)
            .unwrap();

        instance.set_state(n.state.clone());

        self.instance_status_change_notifier.notify_waiters();
    }

    ///
    ///
    ///
    pub fn process_alert(&mut self, n: AlertNotification) {
        let topic = Topic::from_string(n.topic.clone());
        // println!("{:?}", p.device);

        let instance_name = &topic.instance;

        //
        // Create the instance if not already created
        self.create_instance_if_not_exists(instance_name);

        //
        // Instance MUST now exist
        let instance = self
            .structure
            .get_mut_instance(instance_name)
            .ok_or(Error::Wtf)
            .unwrap();

        instance.add_alert(n.into());

        self.instance_status_change_notifier.notify_waiters();
    }

    ///
    /// Process an element creation notification
    ///
    pub fn process_element_creation(&mut self, n: StructuralNotification) -> Result<(), Error> {
        let topic = Topic::from_string(n.topic());

        let instance_name = &topic.instance;

        //
        // Create the instance if not already created
        self.create_instance_if_not_exists(instance_name);

        //
        // Instance MUST now exist
        let instance = self
            .structure
            .get_mut_instance(instance_name)
            .ok_or(Error::Wtf)
            .unwrap();

        match n {
            StructuralNotification::Attribute(_attribute_notification) => {
                let new_attribute = AttributElement::from(_attribute_notification);
                //
                // You have to insert the element in the instance
                if topic.layers_len() == 1 {
                    instance.insert_attribute(topic.first_layer().clone(), new_attribute);
                }
                //
                //
                else {
                    let mut layers = topic.layers.clone();
                    // println!("---------- {:?}", layers);
                    layers.remove(layers.len() - 1);
                    // println!("---------- {:?}", layers);
                    let class = instance.get_mut_class_from_layers(&layers).unwrap();
                    class.insert_attribute(topic.last_layer().clone(), new_attribute);
                }
            }
            StructuralNotification::Interface(_interface_notification) => {
                let new_class = ClassElement::from(_interface_notification);
                //
                // You have to insert the element in the instance
                if topic.layers_len() == 1 {
                    instance.insert_class(topic.first_layer().clone(), new_class);
                }
                //
                //
                else {
                    let mut layers = topic.layers.clone();
                    layers.remove(layers.len() - 1);
                    let class =
                        instance
                            .get_mut_class_from_layers(&layers)
                            .ok_or(Error::InternalLogic(format!(
                                "cannot find class from layer {:?}",
                                &layers
                            )))?;
                    class.insert_class(topic.last_layer().clone(), new_class);
                }
            }
        }

        self.instance_structure_change_notifier.notify_waiters();

        Ok(())
    }

    ///
    ///
    pub fn pack_instance_status(&self) -> Vec<(String, State, Vec<Alert>)> {
        self.structure.pack_instance_status()
    }

    ///
    ///
    ///
    pub fn structure_into_json_value(&self) -> Result<serde_json::Value, Error> {
        serde_json::to_value(&self.structure)
            .map_err(|e| Error::SerializeFailure(format!("{:?}", e)))
    }

    ///
    ///
    pub fn instance_status_change_notifier(&self) -> Arc<Notify> {
        self.instance_status_change_notifier.clone()
    }

    ///
    ///
    pub fn instance_structure_change_notifier(&self) -> Arc<Notify> {
        self.instance_structure_change_notifier.clone()
    }
}
