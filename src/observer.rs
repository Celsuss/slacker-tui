use std::rc::Weak;

pub enum Event { 
    ChangeConversation(String),
}

pub trait Observer{
    fn notify(&self, event: &Event);
}

pub struct Notifier {
    observers: Vec<Weak<dyn Observer>>,
}

impl Notifier {
    pub fn new() -> Self {
        Notifier {
            observers: Vec::new(),
        }
    }

    pub fn add_observer(&mut self, observer: Weak<dyn Observer>) {
        self.observers.push(observer);
    }

    pub fn notify_observers(&self, event: Event) {
        for observer_ptr in &self.observers {
            let observer = observer_ptr.upgrade();
            if observer.is_some(){
                let observer = observer.unwrap().notify(&event);
                // observer.notify(&event);
            }
        }
    }
}
