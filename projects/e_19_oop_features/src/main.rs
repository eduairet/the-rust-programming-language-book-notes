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

    // OOP Design Pattern: State Pattern
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // Using types to achieve the state pattern
    let mut post_v2 = PostV2::new();
    post_v2.add_text("I ate a salad for lunch today");
    let post_v2 = post_v2.request_review();
    let post_v2 = post_v2.approve(); // We need to reassign the post_v2 variable
    assert_eq!("I ate a salad for lunch today", post_v2.content());
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

// OOP Design Pattern: State Pattern
#[allow(dead_code)]
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

#[allow(dead_code)]
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

#[allow(dead_code)]
struct Draft {}

impl State for Draft {
    // Here we need to use the Box<Self> syntax
    // because Rust needs to know how much space every value takes up
    // at compile time, and the size of a Box<T> value
    // depends on the size of the type T.
    // By using Box<Self> we’re saying that the type of self is a struct
    // that implements the State trait,
    // and we’re taking ownership of self,
    // so we’re moving the state value out of the current state value,
    // basically changing the state value in the post to the new state value.

    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

#[allow(dead_code)]
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

#[allow(dead_code)]
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        &_post.content
    }
}

// Using types to achieve the state pattern
struct PostV2 {
    content: String,
}

struct DraftPost {
    content: String,
}

impl PostV2 {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> PostV2 {
        PostV2 {
            content: self.content,
        }
    }
}
