pub mod app;
pub mod events;
pub mod project;
pub mod ui;

#[cfg(test)]
mod tests {

    #[test]
    fn smoke_test() {
        assert_eq!(2 + 2, 4);
    }
}
