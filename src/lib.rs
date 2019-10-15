enum Status {
    Draft,
    PendingReview,
    Approved,
}

pub struct Post {
    status: Status,
    content: String
}

impl Post {
    pub fn new() -> Post {
        Post {
            status: Status::Draft,
            content: String::from(""),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content = String::from(text);
    }

    pub fn request_review(&mut self){
        self.status = Status::PendingReview;
    }

    pub fn approve(&mut self){
        self.status = Status::Approved;
    }

    pub fn content(&self) -> &str {
        match self.status {
           Status::Approved => &self.content,
           _ => ""
        }
    }
}