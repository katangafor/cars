use std::{thread, time};

fn main() {
    let half_second = time::Duration::from_millis(500);
    // UMMMMMMMMMMMMMMMMMMMMMMMMMMM ADDING COMMENT LOL
    // let mut ferrari = build_car(100, 5, None);
    // let mut jetta = build_car(50, 10);
    let mut ferrari = build_car(75, 5, &String::from("Ferrari"));
    let mut jetta = build_car(50, 10, &String::from("Jetta"));

    // a vec of cars
    let mut lane = vec![&mut ferrari, &mut jetta];

    loop {
        for car in &mut lane {
            car.drive();
            car.print_status();
        }
        println!("----------");
        thread::sleep(half_second);
    }

    // loop backwards through the vec
    // if a car is going to hit the car in front of it, reduce it's speed to only come within 5 distance
}

#[derive(Debug)]
struct Car {
    // the car's displacement
    x: u32,
    // how much displacement the car will get in the next iteration
    speed: u32,
    // car's name
    name: String,
}

impl Car {
    // increments the displacement by the speed
    fn drive(&mut self) {
        self.x = self.x + self.speed;
    }
    fn print_status(&self) {
        println!("{} is at {}, going {}", self.name, self.x, self.speed);
    }
}

fn build_car(x: u32, speed: u32, name: &String) -> Car {
    Car {
        x,
        speed,
        name: String::from(name),
    }
}

fn report_distance_to_leader(follower: &Car, leader: &Car) -> u32 {
    leader.x - follower.x
}
