struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    fn new(title: String, description: String, status: String) -> Self {
        match (
            title.is_empty(),
            description.is_empty(),
            title.len() <= 50,
            description.len() <= 500,
            status.as_str(),
        ) {
            (false, false, true, true, "To-Do" | "In Progress" | "Done") => Self {
                title,
                description,
                status,
            },
            (true, _, _, _, _) => panic!("Title cannot be empty"),
            (_, true, _, _, _) => panic!("Description cannot be empty"),
            (_, _, false, _, _) => panic!("Title cannot be longer than 50 characters"),
            (_, _, _, false, _) => panic!("Description cannot be longer than 500 characters"),
            (_, _, _, _, _) => {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new("".into(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), "".into(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 characters")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(overly_long_title(), valid_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 characters")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), overly_long_description(), "To-Do".into());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "Funny".into());
    }

    #[test]
    fn done_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "Done".into());
    }

    #[test]
    fn in_progress_is_allowed() {
        Ticket::new(valid_title(), valid_description(), "In Progress".into());
    }
}
