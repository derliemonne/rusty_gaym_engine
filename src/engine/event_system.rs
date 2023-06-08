use std::default;

use super::*;


pub trait EventT {}


pub trait EventSystemT<E: EventT> : Default {
    fn add_event(&mut self, event: E);
    fn on_trigger(&mut self, event: &E);
}