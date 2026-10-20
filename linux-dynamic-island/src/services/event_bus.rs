use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};

#[derive(Debug, Clone, PartialEq)]
pub enum SystemEvent {
    MediaTrackChanged {
        title: String,
        artist: String,
        album: Option<String>,
        is_playing: bool,
    },
    NotificationReceived {
        id: u32,
        app_name: String,
        summary: String,
        body: String,
        icon: Option<String>,
    },
    BatteryLevelChanged {
        percentage: u8,
        is_charging: bool,
    },
    VolumeChanged {
        percentage: u8,
        is_muted: bool,
    },
    NetworkStatusChanged {
        is_connected: bool,
        ssid: Option<String>,
    },
}

#[derive(Clone, Default)]
pub struct EventBus {
    subscribers: Arc<Mutex<Vec<Sender<SystemEvent>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            subscribers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Subscribe to receive events published on the bus
    pub fn subscribe(&self) -> Receiver<SystemEvent> {
        let (tx, rx) = channel();
        if let Ok(mut subs) = self.subscribers.lock() {
            subs.push(tx);
        }
        rx
    }

    /// Publish an event to all active subscribers
    pub fn publish(&self, event: SystemEvent) {
        if let Ok(mut subs) = self.subscribers.lock() {
            // Retain only healthy subscriber channels
            subs.retain(|sub| sub.send(event.clone()).is_ok());
        }
    }

    /// Number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.lock().map(|s| s.len()).unwrap_or(0)
    }
}
