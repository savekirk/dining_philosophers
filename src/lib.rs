use actix::prelude::*;
use actix::Actor;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

type Address = Arc<Addr<Chopstick>>;
#[derive(Debug)]
pub struct Philosopher {
    pub name: String,
    pub state: String,
    pub left: Option<Address>,
    pub right: Option<Address>,
    left_taken: bool,
    right_taken: bool,
}

impl Philosopher {
    pub fn new(name: &str, left: Address, right: Address) -> Self {
        Philosopher {
            name: name.to_string(),
            state: String::from("Thinking"),
            left: Some(left),
            right: Some(right),
            left_taken: false,
            right_taken: false,
        }
    }
}

impl Actor for Philosopher {
    type Context = Context<Self>;
}
#[derive(Message)]
#[rtype(result = "()")]
enum ChopstickState {
    ChopstickAvailable(ChopstickPosition),
    ChopstickUnAvailable,
}
#[derive(Message)]
#[rtype(result = "()")]
pub enum Action {
    Think,
    Eat,
}

impl Handler<Action> for Philosopher {
    type Result = ();
    fn handle(&mut self, msg: Action, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Action::Eat => {
                if self.right_taken && self.left_taken {
                    println!("{} is eating", self.name);
                    thread::sleep(Duration::from_secs(2));
                    println!("{} has finished eating", self.name);
                    self.left
                        .clone()
                        .unwrap()
                        .do_send(ChopstickAction::Put(ctx.address()));
                    self.right
                        .clone()
                        .unwrap()
                        .do_send(ChopstickAction::Put(ctx.address()));
                    self.left_taken = false;
                    self.right_taken = false;
                    ctx.address().do_send(Action::Think);
                }
            }
            Action::Think => {
                println!("{} is thinking", self.name);
                thread::sleep(Duration::from_secs(5));
                println!("{} has finished thinking", self.name);
                self.left.clone().unwrap().do_send(ChopstickAction::Take(
                    ctx.address(),
                    ChopstickPosition::Left,
                ));
                self.left.clone().unwrap().do_send(ChopstickAction::Take(
                    ctx.address(),
                    ChopstickPosition::Right,
                ))
            }
        }
    }
}

impl Handler<ChopstickState> for Philosopher {
    type Result = ();

    fn handle(&mut self, msg: ChopstickState, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            ChopstickState::ChopstickAvailable(position) => {
                println!("{:} got {:?} chopstick", self.name, position);
                match position {
                    ChopstickPosition::Left => self.left_taken = true,
                    ChopstickPosition::Right => self.right_taken = true,
                }
                if self.left_taken && self.right_taken {
                    ctx.address().do_send(Action::Eat);
                }
            }
            ChopstickState::ChopstickUnAvailable => {
                if self.left_taken {
                    self.left
                        .clone()
                        .unwrap()
                        .do_send(ChopstickAction::Put(ctx.address()));
                    self.left_taken = false;
                    println!("{:} dropped left chopstick", self.name);
                } else if self.right_taken {
                    self.right
                        .clone()
                        .unwrap()
                        .do_send(ChopstickAction::Put(ctx.address()));
                    self.right_taken = false;
                    println!("{:} dropped right chopstick", self.name);
                }
                ctx.address().do_send(Action::Think);
            }
        }
    }
}
#[derive(Debug)]
pub struct Chopstick {
    is_busy: bool,
}

impl Chopstick {
    pub fn new() -> Self {
        Chopstick { is_busy: false }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Debug)]
enum ChopstickAction {
    Take(Addr<Philosopher>, ChopstickPosition),
    Put(Addr<Philosopher>),
}

impl Actor for Chopstick {
    type Context = Context<Self>;
}

impl Handler<ChopstickAction> for Chopstick {
    type Result = ();
    fn handle(&mut self, msg: ChopstickAction, _ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            ChopstickAction::Take(addr, position) => {
                if !self.is_busy {
                    addr.do_send(ChopstickState::ChopstickAvailable(position));
                } else {
                    addr.do_send(ChopstickState::ChopstickUnAvailable)
                }
            }
            ChopstickAction::Put(_) => self.is_busy = false,
        }
    }
}

#[derive(Debug)]
enum ChopstickPosition {
    Left,
    Right,
}
