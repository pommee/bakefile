#[cfg(test)]
mod tests {
    use bake::recepie::{Recepie, RecepieUtil};

    #[test]
    fn test_new_recepie() {
        let recepie = Recepie::new();
        assert_eq!(recepie.name, "");
        assert_eq!(recepie.dependencies.len(), 0);
        assert_eq!(recepie.commands.len(), 0);
    }
}
