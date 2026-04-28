use genesis_tui::core::entity::Entity;
use genesis_tui::core::grid::Position;
use genesis_tui::core::genetics::Genome;

#[test]
fn test_entity_creation() {
    let pos = Position::new(10, 20);
    let genome = Genome::new_random(16);
    let entity = Entity::new(1, pos, genome.clone());

    assert_eq!(entity.id(), 1);
    assert_eq!(*entity.position(), Position::new(10, 20));
    assert_eq!(entity.genome().sequence().len(), 16);
}

#[test]
fn test_entity_mutation_passthrough() {
    let pos = Position::new(0, 0);
    let genome = Genome::new_random(16);
    let mut entity = Entity::new(1, pos, genome);

    let original_dna = entity.genome().sequence().to_vec();
    entity.mutate_genome(1.0);

    assert_ne!(original_dna, entity.genome().sequence());
}
