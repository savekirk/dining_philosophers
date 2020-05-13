use actix::prelude::*;
use actix::Actor;

pub struct Philosopher {
    pub name: String,
    pub state: String,
    pub left: Option<Addr<Chopstick>>,
    pub right: Option<Addr<Chopstick>>,
}

impl Philosopher {
    pub fn new(name: &str) -> Self {
        Philosopher {
            name: name.to_string(),
            state: String::from("Thinking"),
            left: None,
            right: None,
        }
    }
}

impl Actor for Philosopher {
    type Context = Context<Self>;
}
#[derive(Message)]
#[rtype(result = "String")]
pub enum PhilosopherMessage {
    Eat,
    Think,
    ChopstickAvailable(Addr<Chopstick>),
    ChopstickUnAvailable(Addr<Chopstick>),
}

impl Handler<PhilosopherMessage> for Philosopher {
    type Result = String;

    fn handle(&mut self, msg: PhilosopherMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            PhilosopherMessage::Eat => {
                if let Some(_) = self.left {
                    if let Some(_) = self.right {
                        self.state = String::from("Eating")
                    }
                }
            }
            PhilosopherMessage::Think => self.state = String::from("Thinking"),
            PhilosopherMessage::ChopstickAvailable(addr) => {
                if let Some(_) = self.right {
                    self.left = Some(addr);
                    self.state = String::from("Thinking")
                } else if let Some(_) = self.left {
                    self.right = Some(addr);
                    self.state = String::from("Thinking")
                }
            }
            PhilosopherMessage::ChopstickUnAvailable(addr) => {
                if let Some(_) = self.right {
                    addr.do_send(ChopstickAction::Put(ctx.address()))
                } else if let Some(_) = self.left {
                    self.right = Some(addr);
                    self.state = String::from("Thinking")
                }
            }
        }
        println!("{} is Currently {}", self.name, self.state);
        self.state.clone()
    }
}
pub struct Chopstick {
    is_busy: bool,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Take {
    addr: Addr<Philosopher>,
    action: ChopstickAction,
    position: ChopstickPosition,
}
#[derive(Message)]
#[rtype(result = "()")]
pub enum ChopstickAction {
    Take(Addr<Philosopher>),
    Put(Addr<Philosopher>),
}

impl Take {
    pub fn new(
        addr: Addr<Philosopher>,
        action: ChopstickAction,
        position: ChopstickPosition,
    ) -> Self {
        Take {
            addr,
            action,
            position,
        }
    }
}
impl Actor for Chopstick {
    type Context = SyncContext<Self>;
}

impl Handler<ChopstickAction> for Chopstick {
    type Result = ();
    fn handle(&mut self, msg: ChopstickAction, ctx: &mut SyncContext<Self>) -> Self::Result {
        match msg {
            ChopstickAction::Take(addr) => {
                if !self.is_busy {
                    self.is_busy = true;
                    addr.do_send(PhilosopherMessage::ChopstickAvailable(ctx.address()))
                } else {
                    addr.do_send(PhilosopherMessage::ChopstickUnAvailable(ctx.address()))
                }
            }
            ChopstickAction::Put(addr) => self.is_busy = false,
        }
    }
}

pub enum ChopstickPosition {
    LEFT,
    RIGHT,
}
