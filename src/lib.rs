use actix::prelude::*;
use std::sync::Arc;
use std::time::Duration;

type Address = Arc<Addr<Chopstick>>;

pub struct Philosopher {
    pub name: String,
    pub state: PhilosopherState,
    pub left: Address,
    pub right: Address,
}

#[derive(Debug)]
pub enum PhilosopherState {
    Waiting,
    Eating,
    Thinking,
    Hungry,
    WaitingForOtherChopstick,
    FirstChopStickDenied,
}

impl Philosopher {
    pub fn new(name: &str, left: Address, right: Address) -> Self {
        Philosopher {
            name: name.to_string(),
            state: PhilosopherState::Waiting,
            left,
            right,
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
    ChopstickUnAvailable(ChopstickPosition),
}
#[derive(Message)]
#[rtype(result = "()")]
pub enum Action {
    Think,
    Eat,
}

impl Handler<Action> for Philosopher {
    type Result = ();
    fn handle(&mut self, msg: Action, ctx: &mut Context<Self>) -> Self::Result {
        match msg {
            Action::Eat => match self.state {
                PhilosopherState::Thinking => {
                    chopstick_command(
                        self.left.clone(),
                        ChopstickAction::Take(ctx.address(), ChopstickPosition::Left),
                    );
                    chopstick_command(
                        self.right.clone(),
                        ChopstickAction::Take(ctx.address(), ChopstickPosition::Right),
                    );
                    self.state = PhilosopherState::Hungry;
                }
                _ => {}
            },
            Action::Think => {
                match self.state {
                    PhilosopherState::Waiting => {
                        println!("{} is thinking", self.name);
                    }
                    PhilosopherState::Eating => {
                        println!("{} dropped both chopsticks and started thinking", self.name);
                        chopstick_command(self.left.clone(), ChopstickAction::Put);
                        chopstick_command(self.right.clone(), ChopstickAction::Put);
                    }
                    _ => {}
                };
                self.state = PhilosopherState::Thinking;
                ctx.notify_later(Action::Eat, Duration::from_secs(5));
            }
        }
    }
}

impl Handler<ChopstickState> for Philosopher {
    type Result = ();

    fn handle(&mut self, msg: ChopstickState, ctx: &mut Self::Context) -> Self::Result {
        match self.state {
            PhilosopherState::Hungry => match msg {
                ChopstickState::ChopstickAvailable(_) => {
                    self.state = PhilosopherState::WaitingForOtherChopstick
                }
                ChopstickState::ChopstickUnAvailable(_) => {
                    self.state = PhilosopherState::FirstChopStickDenied
                }
            },
            PhilosopherState::WaitingForOtherChopstick => match msg {
                ChopstickState::ChopstickAvailable(_) => {
                    println!("{} got both chopsticks and start to eat", self.name);
                    self.state = PhilosopherState::Eating;
                    start_eating(ctx, Duration::from_secs(2));
                }
                ChopstickState::ChopstickUnAvailable(position) => {
                    println!(
                        "{} did not get {:#?} chopsticks and cannot eat",
                        self.name, position
                    );
                    if position == ChopstickPosition::Left {
                        chopstick_command(self.left.clone(), ChopstickAction::Put);
                    } else {
                        chopstick_command(self.right.clone(), ChopstickAction::Put);
                    }
                    self.state = PhilosopherState::Thinking;
                    start_thinking(ctx, Duration::from_millis(10));
                }
            },
            PhilosopherState::FirstChopStickDenied => {
                match msg {
                    ChopstickState::ChopstickAvailable(position) => {
                        if position == ChopstickPosition::Left {
                            chopstick_command(self.left.clone(), ChopstickAction::Put);
                        } else {
                            chopstick_command(self.right.clone(), ChopstickAction::Put);
                        }
                    }
                    ChopstickState::ChopstickUnAvailable(_) => {}
                }
                self.state = PhilosopherState::Thinking;
                start_thinking(ctx, Duration::from_millis(10));
            }
            _ => {}
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
enum ChopstickAction {
    Take(Addr<Philosopher>, ChopstickPosition),
    Put,
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
                    self.is_busy = true;
                } else {
                    addr.do_send(ChopstickState::ChopstickUnAvailable(position))
                }
            }
            ChopstickAction::Put => self.is_busy = false,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum ChopstickPosition {
    Left,
    Right,
}

fn start_thinking(ctx: &mut Context<Philosopher>, duration: Duration) {
    ctx.notify_later(Action::Eat, duration);
}

fn start_eating(ctx: &mut Context<Philosopher>, duration: Duration) {
    ctx.notify_later(Action::Think, duration);
}

fn chopstick_command(chopstick: Address, action: ChopstickAction) {
    chopstick.do_send(action);
}
