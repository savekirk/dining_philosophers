use actix::{Actor, System};
use dinning_philosophers::{Action, Chopstick, Philosopher};
use std::sync::Arc;

fn main() -> std::io::Result<()> {
    System::run(|| {
        let mut chopsticks = Vec::with_capacity(5);
        for _ in 0..5 {
            chopsticks.push(Arc::new(Chopstick::new().start()));
        }

        let philosophers = vec!["Zeno", "Seneca", "Cato", "Epictetus", "Aurelius"];
        for i in 0..5 {
            let philosopher = Philosopher::new(
                philosophers[i],
                chopsticks[i].clone(),
                chopsticks[i % 5].clone(),
            )
            .start();
            philosopher.do_send(Action::Think);
        }
        System::current().stop();
    })
}
