use crate::Engine;

pub fn invoke(engine: &mut Engine, tokens: &[&str]) {
    if let Some((depth, _)) = tokens.split_first()
        && let Ok(depth) = depth.parse::<usize>()
    {
        engine.search_depth(depth);
    }
}
