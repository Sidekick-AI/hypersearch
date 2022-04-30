#[cfg(test)]
mod tests {
    use hypersearch_codegen::hyperparameters;

    #[test]
    fn test_hyperparameter_search() {
        #[hyperparameters]
        struct Hyperparams {
            lr: Vec<f32>,
            embed_size: Vec<u16>,
        }

        let hyperparams = Hyperparams {
            lr: vec![1e-6, 1e-3],
            embed_size: vec![128, 1024],
        };

        assert_eq!(hyperparams.permutations().count(), 4);
    }
}
