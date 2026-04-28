use genesis_tui::core::genetics::Genome;

#[test]
fn test_genome_initialization() {
    let dna = Genome::new_random(16); // 16-byte DNA sequence
    assert_eq!(dna.sequence().len(), 16);
}

#[test]
fn test_genome_mutation() {
    let mut dna = Genome::new_random(16);
    let original = dna.sequence().to_vec();

    // Force a 100% mutation rate across the sequence bounds
    dna.mutate(1.0);

    assert_ne!(
        original,
        dna.sequence(),
        "DNA must diverge from its origin after heavy mutation"
    );
}
