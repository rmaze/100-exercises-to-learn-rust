mod description;
mod status;
mod title;

pub use description::TicketDescription;
pub use status::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

impl Ticket {
    pub fn new(title: TicketTitle, description: TicketDescription, status: Status) -> Self {
        Ticket {
            title,
            description,
            status,
        }
    }
}
