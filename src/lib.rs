pub use sha2;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use sha2::Digest;
        let _ = sha2::Sha256::new();
    }
}
