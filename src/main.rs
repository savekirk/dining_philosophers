use actix::{Actor, Arbiter, SyncArbiter, System};
use dinning_philosophers::{Chopstick, Philosopher, PhilosopherMessage, Take};
fn main() {
    let system = System::new("Philosophers");

    let chopsticks = SyncArbiter::start(5, || Chopstick {});

    let zeno = Philosopher::new("Zeno").start();
    let seneca = Philosopher::new("Seneca").start();
    let cato = Philosopher::new("Cato, Marcus Porcius").start();
    let epictetus = Philosopher::new("Epictetus").start();
    let marcus = Philosopher::new("Marcus Aurelius").start();

    zeno.do_send(PhilosopherMessage::Eat);
    marcus.do_send(PhilosopherMessage::Think);
    seneca.do_send(PhilosopherMessage::Eat);
    cato.do_send(PhilosopherMessage::Think);
    epictetus.do_send(PhilosopherMessage::Think);
    chopsticks.do_send(Take::new(zeno.clone()));
    chopsticks.do_send(Take::new(zeno.clone()));
    system.run();

    // match res {
    //     Ok(res) => println!("Professor is {}", res),
    //     _ => println!("Failed to get current state")
    // }
}
