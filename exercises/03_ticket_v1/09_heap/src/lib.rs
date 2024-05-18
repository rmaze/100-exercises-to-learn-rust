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
    fn string_size() {
        // The size of a `String` is the size of two pointers (for the buffer and the capacity)
        // plus the size of a `usize` (for the length).
        assert_eq!(size_of::<String>(), 24); // assuming a 64-bit architecture
    }

    #[test]
    fn ticket_size() {
        // The size of `Ticket` is the sum of the sizes of its fields (`String` is a pointer
        // to the actual string data, so its size is not included in the struct size calculation).
        // Each `String` field has a size of 24 bytes, so the total size is 24 + 24 + 24 = 72.
        assert_eq!(size_of::<Ticket>(), 72); // assuming a 64-bit architecture
    }
}
