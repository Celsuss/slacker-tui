
// #[derive(Copy)]
pub enum Event { 
    Tick,
    Quit,
    ChangeConversation(String),
}

pub trait Observer{
    fn notify(&self, event: &Event);
}

pub struct Notifier {
    observers: Vec<Box<dyn Observer>>,
}

impl Notifier {
    pub fn new() -> Self {
        Notifier {
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn notify_observers(&self, event: Event) {
        for observer in &self.observers {
            observer.notify(&event);
        }
    }
}
