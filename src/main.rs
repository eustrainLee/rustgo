use std::{thread::sleep, time};

mod gogo;

fn main() {
    let r = gogo::plus_(1,2);
    println!("{r}");
    let a = gogo::start_("a".to_string());
    let b = gogo::start_("b".to_string());
    for _ in 0..10 {
        let ra = gogo::get_(a);
        let rb = gogo::get_(b);
        println!("ra:{ra}, rb:{rb}");
        sleep(time::Duration::from_secs(2));
    }
}