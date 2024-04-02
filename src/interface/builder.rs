



pub struct Builder {
    name: String,
    idn: Box<dyn IdentityProvider>,
    states: Box<dyn fsm::States>,
    subscriber: Box<dyn Subscriber>
}



