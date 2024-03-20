

#[async_trait]
pub trait MessageProcessor : Send {



    async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest>;

    async fn process(&self, data: &SharedData, msg: &SubscriptionMessage);

}

// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------
// ------------------------------------------------------------------------------------------------

/// Message handler
/// 
struct Listener {
    
    /// Shared state data
    data: SharedData,

    /// 
    impls: Box<dyn HandlerImplementations>,
    
    // links interface handles
    links: LinkedList<LinkInterfaceHandle>
}

impl Listener {
    
    fn new(data: SharedData, impls: Box<dyn HandlerImplementations>) -> Listener {

        return Listener {
            data: data,
            impls: impls,
            links: LinkedList::new()
        }
    }

    ///
    ///
    pub async fn get_subscription_requests(&self) -> Vec<SubscriptionRequest> {
        return self.impls.get_subscription_requests().await;
    }

    ///
    /// 
    pub fn add_link(&mut self, link: LinkInterfaceHandle) {
        self.links.push_back(link);
    }
    
    ///
    ///
    pub async fn run_once(&mut self) {
        for link in self.links.iter_mut() {
            let msg = link.rx.recv().await;
            match msg {
                Some(msg) => {
                    self.impls.process(&self.data, &msg).await;
                },
                None => {
                    // do nothing
                }
            }
        }
    }

}
