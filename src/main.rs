use std::{thread, time};

fn main() {
    let half_second = time::Duration::from_millis(500);
    // UMMMMMMMMMMMMMMMMMMMMMMMMMMM ADDING COMMENT LOL

    let mut ferrari = build_car(200, 5, &String::from("Ferrari"));
    let mut jetta = build_car(180, 10, &String::from("Jetta"));
    let mut civic = build_car(170, 10, &String::from("Civic"));
    let mut cup = build_car(150, 10, &String::from("Cup"));
    let mut miata = build_car(130, 10, &String::from("Miata"));
    let mut lexus = build_car(120, 10, &String::from("Lexus"));

    let dist = report_distance_to_leader(&jetta, &ferrari);
    println!("dist is {}", dist);

    let mut lane = vec![
        &mut ferrari,
        &mut jetta,
        &mut civic,
        &mut cup,
        &mut miata,
        &mut lexus,
    ];

    let iter1 = lane.iter();
    let iter2 = lane.iter().skip(1);

    let zipped = iter1.zip(iter2);

    for e in zipped {
        println!(
            "{} leads {} by {}",
            e.0.name,
            e.1.name,
            report_distance_to_leader(e.1, e.0)
        );
    }

    // loop {
    //     for car in &mut lane {
    //         car.drive();
    //         car.print_status();
    //     }
    //     println!("----------");
    //     thread::sleep(half_second);
    // }
}

#[derive(Debug)]
struct Car {
    // the car's displacement
    x: i32,
    // how much displacement the car will get in the next iteration
    speed: i32,
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
    fn drive_but_dont_crash(&mut self, leader: &Car) {
        let comfortable_follow_distance = leader.x + leader.speed - 5;
        let expected_self_position = self.x + self.speed;
        // if you're gonna get too close to the car in front of you, adjust your
        // speed so that you'll get end up at the right distance
        if expected_self_position > comfortable_follow_distance {
            let appropriate_speed_difference = expected_self_position - comfortable_follow_distance;
            self.speed -= appropriate_speed_difference;
        }
        self.x = self.x + self.speed;
    }
}

fn build_car(x: i32, speed: i32, name: &String) -> Car {
    Car {
        x,
        speed,
        name: String::from(name),
    }
}

fn report_distance_to_leader(follower: &Car, leader: &Car) -> i32 {
    leader.x - follower.x
}