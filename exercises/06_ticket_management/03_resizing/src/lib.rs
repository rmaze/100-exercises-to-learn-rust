#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(2);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 2);

        v.push(3); // beyond capacity, needs to resize

        // The standard library typically doubles the capacity, but this is not guaranteed.
        // However, for most current implementations, the capacity will be doubled.
        assert_eq!(v.capacity(), 4);
    }
}
