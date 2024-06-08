fn main() {
    // Encapsulation
    let mut avg = AverageCollection::new();
    avg.add(10);
    avg.add(20);
    avg.add(30);
    println!("Average: {}", avg.average());
    avg.remove();
    println!("Average: {}", avg.average());

    // Inheritance and polymorphism
    let _screen = Screen {
        components: vec![
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("No"),
                    String::from("Maybe"),
                ],
            }),
        ],
    };

    _screen.run();
}

// Encapsulation
struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}
impl AverageCollection {
    pub fn new() -> AverageCollection {
        AverageCollection {
            list: Vec::new(),
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Inheritance
trait Draw {
    fn draw(&self);
}

struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    #[allow(dead_code)]
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[allow(dead_code)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing SelectBox");
    }
}

#[allow(dead_code)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing Button");
    }
}