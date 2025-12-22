#[cfg(test)]
mod tests {
    use crate::map::MapManager;
    use crate::map::chunk::CHUNK_SIZE;
    use crate::entity::Pos;

    #[test]
    fn test_chunk_generation() {
        let mut map = MapManager::new();
        map.generate_chunk_if_needed(0, 0);

        // Check if chunk exists
        assert!(map.chunks.contains_key(&(0, 0)));

        // Check if it has some floors
        let chunk = map.chunks.get(&(0, 0)).unwrap();
        let floor_count = chunk.tiles.iter().filter(|t| t.char == '.').count();
        assert!(floor_count > 0);
    }

    #[test]
    fn test_connectivity() {
        let mut map = MapManager::new();
        // Generate (0,0) and (1,0)
        map.generate_chunk_if_needed(0, 0);
        map.generate_chunk_if_needed(1, 0);

        let chunk0 = map.chunks.get(&(0, 0)).unwrap();
        let chunk1 = map.chunks.get(&(1, 0)).unwrap();

        // Check east border of chunk0 matches west border of chunk1
        for y in 0..CHUNK_SIZE {
            let t0 = chunk0.get_tile(CHUNK_SIZE - 1, y).unwrap();
            let t1 = chunk1.get_tile(0, y).unwrap();

            if t0.char == '.' {
                assert_eq!(t1.char, '.', "Chunk (1,0) west edge mismatch at y={} with (0,0) east edge", y);
            }
            if t1.char == '.' {
                assert_eq!(t0.char, '.', "Chunk (0,0) east edge mismatch at y={} with (1,0) west edge", y);
            }
        }
    }

    #[test]
    fn test_infinite_coordinates() {
        let mut map = MapManager::new();
        // Generate chunk at negative coordinates
        map.generate_chunk_if_needed(-5, -5);

        let chunk = map.chunks.get(&(-5, -5));
        assert!(chunk.is_some());

        let pos = Pos::new(-5 * CHUNK_SIZE + 10, -5 * CHUNK_SIZE + 10);
        let tile = map.get_tile(pos);
        // Should not panic and return something valid (wall or floor)
        assert!(tile.char == '#' || tile.char == '.');
    }
}
