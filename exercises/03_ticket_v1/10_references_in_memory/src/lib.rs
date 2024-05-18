pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn u16_ref_size() {
        assert_eq!(size_of::<&u16>(), 8); // On a 64-bit architecture, a reference is 8 bytes
    }

    #[test]
    fn u64_mut_ref_size() {
        assert_eq!(size_of::<&u64>(), 8); // On a 64-bit architecture, a reference is 8 bytes
    }

    #[test]
    fn ticket_ref_size() {
        assert_eq!(size_of::<&Ticket>(), 8); // On a 64-bit architecture, a reference is 8 bytes
    }
}
