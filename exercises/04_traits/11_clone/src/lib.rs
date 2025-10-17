// TODO: add the necessary `Clone` implementations (and invocations)
//  to get the code to compile.

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let summary = ticket.clone().summary();
    (ticket, summary)
    
}
impl Clone for Ticket  {
    fn clone(&self) -> Self {
        Ticket { 
            title: self.title.clone(), 
            description: self.description.clone(), 
            status: self.status.clone() 
        }
    }
}


pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

#[derive(Clone)]
pub struct Summary {
    pub title: String,
    pub status: String,
}
