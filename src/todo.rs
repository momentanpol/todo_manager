
#[derive(Debug)]
pub struct Todo {
    id: Option<usize>,
    message: Option<String>,
    created: Option<String>,
    updated: Option<String>,
    keywords: Option<Vec<String>>,
    until: Option<String>,
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub struct TodoBuilder {
    msg: Option<String>,
    id: Option<usize>,
    created: Option<String>,
    updated: Option<String>,
    until: Option<String>,
    keywords: Option<Vec<String>>,
}

impl TodoBuilder {
    pub fn new() -> TodoBuilder {
        TodoBuilder {
            msg: None,
            id: None,
            created: None,
            updated: None,
            until: None,
            keywords: None,
        }
    }

    pub fn message(mut self, message: String) -> TodoBuilder {
        self.msg = Some(message);
        self
    }

    pub fn id(mut self, id: usize) -> TodoBuilder {
        self.id = Some(id);
        self
    }

    pub fn until(mut self, until: Option<String>) -> TodoBuilder {
        self.until = until;
        self
    }

    pub fn build(self) -> Todo {
        Todo {
            message: self.msg,
            id:  self.id,
            created: self.created, 
            updated: self.updated,
            until: self.until,
            keywords: self.keywords,
        }
    }
}
