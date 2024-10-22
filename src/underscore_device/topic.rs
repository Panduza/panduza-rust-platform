///
///
///
#[derive(Debug)]
pub struct Topic {
    pub _namespace: String,
    pub _host: String,
    pub device: String,
    pub layers: Vec<String>,
}

impl Topic {
    pub fn from_string<A: Into<String>>(topic: A) -> Self {
        // Split the topic
        let topic_string = topic.into();
        let mut layers: Vec<&str> = topic_string.split('/').collect();

        //
        //
        let mut namespace_parts: Vec<String> = Vec::new();
        while !layers.is_empty() {
            {
                let layer = layers.get(0).unwrap();
                if *layer == "pza" {
                    break;
                }
                namespace_parts.push(layer.to_string());
            }
            layers.remove(0);
        }

        // Remove pza
        layers.remove(0);

        //
        //
        let namespace = namespace_parts.join("/");
        let host = layers.remove(0).to_string();
        let device = layers.remove(0).to_string();

        Self {
            _namespace: namespace,
            _host: host,
            device,
            layers: layers.into_iter().map(|l| l.to_string()).collect(),
        }
    }
}
