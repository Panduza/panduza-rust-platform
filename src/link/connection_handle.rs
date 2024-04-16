use std::collections::LinkedList;

use tokio::sync::mpsc;

use crate::subscription::{self, Filter};


/// Link handle for the connection
///
pub struct ConnectionHandle
{
    /// Channel to send messages to the interface
    tx: mpsc::Sender<subscription::Message>,

    /// List of filters
    filters: LinkedList<subscription::Filter>,
}

impl ConnectionHandle {
    pub fn new(tx: mpsc::Sender<subscription::Message>, filters: LinkedList<subscription::Filter>) -> ConnectionHandle {
        return ConnectionHandle {
            tx: tx,
            filters: filters,
        }
    }

    #[inline(always)]
    pub fn tx(&self) -> &mpsc::Sender<subscription::Message> {
        return &self.tx;
    }


    #[inline(always)]
    pub fn filters(&self) -> &LinkedList<subscription::Filter> {
        return &self.filters;
    }

}