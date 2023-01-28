pub struct ThreadManager;

pub struct Thread {
    title: String,
    body: String,
    created_by: String,
    created_on: String,
}

impl Thread {
    pub fn new(
        title: &str, 
        body: &str, 
        created_by: &str, 
        created_on: &str
    ) -> ThreadManager {
        todo!()
    }
}
