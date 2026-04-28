use genesis_tui::core::entity::Entity;
use genesis_tui::core::grid::Position;
use genesis_tui::core::genetics::Genome;

#[test]
fn test_entity_creation() {
    let pos = Position::new(10.0, 20.0);
    let genome = Genome::new_random(16);
    let entity = Entity::new(1, pos, genome.clone());

    assert_eq!(entity.id(), 1);
    assert_eq!(*entity.position(), Position::new(10.0, 20.0));
    assert_eq!(entity.genome().sequence().len(), 16);
}

#[test]
fn test_entity_mutation_passthrough() {
    let pos = Position::new(0.0, 0.0);
    let genome = Genome::new_random(16);
    let mut entity = Entity::new(1, pos, genome);

    let original_dna = entity.genome().sequence().to_vec();
    entity.mutate_genome(1.0);

    assert_ne!(original_dna, entity.genome().sequence());
}

#[test]
fn test_entity_electrification() {
    let pos = Position::new(0.0, 0.0);
    let genome = Genome::new_random(16);
    let mut entity = Entity::new(1, pos, genome);

    assert!(!entity.is_electrified());
    assert!(entity.get_render_effect(0).is_none());

    entity.electrify(10);
    assert!(entity.is_electrified());

    let effect_t10 = entity.get_render_effect(10).unwrap();
    let effect_t15 = entity.get_render_effect(15).unwrap();

    assert_eq!(effect_t10.0, "z");
    assert_ne!(effect_t10, effect_t15); // The effect animates over time

    entity.cure_electrification();
    assert!(!entity.is_electrified());
    assert!(entity.get_render_effect(20).is_none());
}

#[test]
fn test_entity_mouse_interaction() {
    let pos = Position::new(0.0, 0.0);
    let genome = Genome::new_random(16);
    let mut entity = Entity::new(1, pos, genome);

    entity.interact(100);
    
    // Test that the render effect returns the shimmer array 
    let effect_t100 = entity.get_render_effect(100).unwrap();
    let effect_t105 = entity.get_render_effect(105).unwrap();
    
    assert!(effect_t100.0 == "+" || effect_t100.0 == "x" || effect_t100.0 == "*" || effect_t100.0 == "." || effect_t100.0 == " ");
    
    // Prove it decays and disappears after 30 ticks
    assert!(entity.get_render_effect(131).is_none());
}
