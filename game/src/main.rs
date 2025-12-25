use engine::EngineContext;

/// Temporary game state for testing.
/// This will later hold your object model, systems, etc.


fn main() {
    // ---- Create engine context (game owns this) ----
    let ctx = EngineContext::new();
    ctx.run();
}
