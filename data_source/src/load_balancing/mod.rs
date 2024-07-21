pub mod least_connections;
pub mod round_robin;

pub use least_connections::LeastConnectionsLoadBalancingStrategy;
pub use round_robin::RoundRobinLoadBalancingStrategy;

#[async_trait::async_trait]
pub trait LoadBalancingStrategy {
    async fn select_listener(&mut self, listeners: &[String]) -> String;
    fn update_state(&mut self, listener: &String, busy: bool);
}

#[derive(Clone)]
pub struct BaseLoadBalancing {
    busy_streams: std::collections::HashMap<String, bool>,
}

impl BaseLoadBalancing {
    pub fn new(listeners: &[String]) -> Self {
        let busy_streams = listeners.iter().cloned().map(|l| (l, false)).collect();
        Self { busy_streams }
    }

    pub fn mark_busy(&mut self, listener: &String) {
        if let Some(busy) = self.busy_streams.get_mut(listener) {
            *busy = true;
        }
    }

    pub fn mark_ready(&mut self, listener: &String) {
        if let Some(busy) = self.busy_streams.get_mut(listener) {
            *busy = false;
        }
    }

    pub fn is_ready(&self, listener: &String) -> bool {
        self.busy_streams.get(listener).cloned().unwrap_or(false) == false
    }

    pub fn get_ready_listener<'a>(&self, listeners: &'a [String]) -> Option<&'a String> {
        listeners.iter().find(|&&ref listener| self.is_ready(listener))
    }
}
