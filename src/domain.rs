pub struct Dictionary{
    name: String,
    content: Vec<String>
}

impl Dictionary {
    pub fn new(name: String,content: Vec<String>) -> Dictionary{
        Dictionary { name: (name), content: (content) }
    }

    pub fn content(&self) -> &Vec<String>{
        return &self.content;
    }
}