pub mod tile;
pub mod chunk;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet, VecDeque};
use macroquad::rand; // Import macroquad's rand
use crate::map::chunk::{Chunk, CHUNK_SIZE};
use crate::map::tile::Tile;
use crate::entity::Pos;

pub struct MapManager {
    pub chunks: HashMap<(i32, i32), Chunk>,
}

impl MapManager {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
        }
    }

    pub fn get_tile(&self, pos: Pos) -> Tile {
        let chunk_x = pos.x.div_euclid(CHUNK_SIZE);
        let chunk_y = pos.y.div_euclid(CHUNK_SIZE);

        let local_x = pos.x.rem_euclid(CHUNK_SIZE);
        let local_y = pos.y.rem_euclid(CHUNK_SIZE);

        if let Some(chunk) = self.chunks.get(&(chunk_x, chunk_y)) {
            if let Some(tile) = chunk.get_tile(local_x, local_y) {
                return *tile;
            }
        }

        Tile::wall()
    }

    pub fn generate_chunk_if_needed(&mut self, chunk_x: i32, chunk_y: i32) {
        if self.chunks.contains_key(&(chunk_x, chunk_y)) {
            return;
        }

        let mut connections = Vec::new();

        // Check neighbors to find entry points
        // North
        if let Some(north) = self.chunks.get(&(chunk_x, chunk_y - 1)) {
            for x in 0..CHUNK_SIZE {
                if north.get_tile(x, CHUNK_SIZE - 1).map_or(false, |t| t.char == '.') {
                    connections.push(Pos::new(x, 0));
                }
            }
        }
        // South
        if let Some(south) = self.chunks.get(&(chunk_x, chunk_y + 1)) {
            for x in 0..CHUNK_SIZE {
                if south.get_tile(x, 0).map_or(false, |t| t.char == '.') {
                    connections.push(Pos::new(x, CHUNK_SIZE - 1));
                }
            }
        }
        // West
        if let Some(west) = self.chunks.get(&(chunk_x - 1, chunk_y)) {
            for y in 0..CHUNK_SIZE {
                if west.get_tile(CHUNK_SIZE - 1, y).map_or(false, |t| t.char == '.') {
                    connections.push(Pos::new(0, y));
                }
            }
        }
        // East
        if let Some(east) = self.chunks.get(&(chunk_x + 1, chunk_y)) {
            for y in 0..CHUNK_SIZE {
                if east.get_tile(0, y).map_or(false, |t| t.char == '.') {
                    connections.push(Pos::new(CHUNK_SIZE - 1, y));
                }
            }
        }

        let mut chunk = Chunk::new(chunk_x, chunk_y);

        let mut walkers = connections.clone();
        if walkers.is_empty() {
             walkers.push(Pos::new(CHUNK_SIZE / 2, CHUNK_SIZE / 2));
        }

        let total_budget = 300;
        let min_steps_per_walker = 50;
        let steps_per_walker = (total_budget / walkers.len().max(1)).max(min_steps_per_walker);

        // --- 1. Random Walk Generation ---
        for start_pos in walkers.iter() {
            let mut curr = *start_pos;
            chunk.set_tile(curr.x, curr.y, Tile::floor());

            for _ in 0..steps_per_walker {
                let dir = rand::gen_range(0, 4);
                let (dx, dy) = match dir {
                    0 => (0, -1), 1 => (0, 1), 2 => (-1, 0), 3 => (1, 0), _ => (0, 0),
                };

                let next_x = curr.x + dx;
                let next_y = curr.y + dy;

                if next_x >= 0 && next_x < CHUNK_SIZE && next_y >= 0 && next_y < CHUNK_SIZE {
                    if self.is_move_allowed(&chunk, chunk_x, chunk_y, next_x, next_y) {
                        curr = Pos::new(next_x, next_y);
                        chunk.set_tile(curr.x, curr.y, Tile::floor());
                    }
                }
            }
        }

        // --- 2. Ensure Connectivity (Merge Islands) ---
        // If we have multiple starting points (connections), we must ensure they are all connected.
        if !connections.is_empty() {
            // Find all connected components of floor tiles
            // Simple BFS from the first connection.
            // If other connections are not reached, carve a path to them.

            // We iterate through all connections. If a connection is not visited by the BFS starting from the first connection,
            // we carve a path from a visited tile to that unvisited connection.

            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();

            // Start BFS from the first connection
            if let Some(first) = connections.first() {
                queue.push_back(*first);
                visited.insert(*first);
            }

            while let Some(pos) = queue.pop_front() {
                for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let nx = pos.x + dx;
                    let ny = pos.y + dy;
                    if nx >= 0 && nx < CHUNK_SIZE && ny >= 0 && ny < CHUNK_SIZE {
                        let n_pos = Pos::new(nx, ny);
                        if !visited.contains(&n_pos) {
                            if let Some(tile) = chunk.get_tile(nx, ny) {
                                if tile.char == '.' {
                                    visited.insert(n_pos);
                                    queue.push_back(n_pos);
                                }
                            }
                        }
                    }
                }
            }

            // Check which connections were not visited
            for conn in &connections {
                if !visited.contains(conn) {
                    // This connection is isolated. Connect it to the main component.
                    // Pick a random visited tile as start
                    if let Some(start) = visited.iter().next() { // Just picking one is fine, or random
                         Self::carve_organic_path(self, &mut chunk, chunk_x, chunk_y, *start, *conn);

                         // Update visited set after carving?
                         // Technically the new path connects them. For simplicity, we just carve.
                         // Ideally we should re-run BFS or just assume it's fine.
                         // Since we carve from a visited tile to the target, the target effectively becomes visited.
                         // But we might need to connect multiple islands.
                         // Let's just carve from *any* visited node to this `conn`.
                    }
                }
            }
        }

        // --- 3. Fix Dead Ends (Force Exit) ---
        let has_north_exit = !self.chunks.contains_key(&(chunk_x, chunk_y - 1)) && (0..CHUNK_SIZE).any(|x| chunk.get_tile(x, 0).map_or(false, |t| t.char == '.'));
        let has_south_exit = !self.chunks.contains_key(&(chunk_x, chunk_y + 1)) && (0..CHUNK_SIZE).any(|x| chunk.get_tile(x, CHUNK_SIZE - 1).map_or(false, |t| t.char == '.'));
        let has_west_exit = !self.chunks.contains_key(&(chunk_x - 1, chunk_y)) && (0..CHUNK_SIZE).any(|y| chunk.get_tile(0, y).map_or(false, |t| t.char == '.'));
        let has_east_exit = !self.chunks.contains_key(&(chunk_x + 1, chunk_y)) && (0..CHUNK_SIZE).any(|y| chunk.get_tile(CHUNK_SIZE - 1, y).map_or(false, |t| t.char == '.'));

        let has_any_exit = has_north_exit || has_south_exit || has_west_exit || has_east_exit;

        if !has_any_exit {
            let mut potential_edges = Vec::new();
            if !self.chunks.contains_key(&(chunk_x, chunk_y - 1)) { potential_edges.push(0); }
            if !self.chunks.contains_key(&(chunk_x, chunk_y + 1)) { potential_edges.push(1); }
            if !self.chunks.contains_key(&(chunk_x - 1, chunk_y)) { potential_edges.push(2); }
            if !self.chunks.contains_key(&(chunk_x + 1, chunk_y)) { potential_edges.push(3); }

            if !potential_edges.is_empty() {
                let target_idx = rand::gen_range(0, potential_edges.len());
                let target_edge = potential_edges[target_idx];

                // Pick a random floor tile as start to ensure connectivity
                let mut floor_tiles = Vec::new();
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        if chunk.get_tile(x, y).map_or(false, |t| t.char == '.') {
                            floor_tiles.push(Pos::new(x, y));
                        }
                    }
                }

                let start_pos = if !floor_tiles.is_empty() {
                    floor_tiles[rand::gen_range(0, floor_tiles.len())]
                } else {
                    Pos::new(CHUNK_SIZE/2, CHUNK_SIZE/2)
                };

                // Determine a target on the edge
                let target_pos = match target_edge {
                    0 => Pos::new(start_pos.x, 0), // North
                    1 => Pos::new(start_pos.x, CHUNK_SIZE - 1), // South
                    2 => Pos::new(0, start_pos.y), // West
                    3 => Pos::new(CHUNK_SIZE - 1, start_pos.y), // East
                    _ => start_pos
                };

                Self::carve_organic_path(self, &mut chunk, chunk_x, chunk_y, start_pos, target_pos);
            }
        }

        self.chunks.insert((chunk_x, chunk_y), chunk);
    }

    fn is_move_allowed(&self, _chunk: &Chunk, chunk_x: i32, chunk_y: i32, next_x: i32, next_y: i32) -> bool {
        // North edge check
        if next_y == 0 {
            if let Some(north) = self.chunks.get(&(chunk_x, chunk_y - 1)) {
                if north.get_tile(next_x, CHUNK_SIZE - 1).map_or(false, |t| t.char == '#') { return false; }
            }
        }
        // South edge check
        if next_y == CHUNK_SIZE - 1 {
            if let Some(south) = self.chunks.get(&(chunk_x, chunk_y + 1)) {
                if south.get_tile(next_x, 0).map_or(false, |t| t.char == '#') { return false; }
            }
        }
        // West edge check
        if next_x == 0 {
            if let Some(west) = self.chunks.get(&(chunk_x - 1, chunk_y)) {
                if west.get_tile(CHUNK_SIZE - 1, next_y).map_or(false, |t| t.char == '#') { return false; }
            }
        }
        // East edge check
        if next_x == CHUNK_SIZE - 1 {
            if let Some(east) = self.chunks.get(&(chunk_x + 1, chunk_y)) {
                if east.get_tile(0, next_y).map_or(false, |t| t.char == '#') { return false; }
            }
        }
        true
    }

    // Carves a "wiggly" path from start to target
    fn carve_organic_path(&self, chunk: &mut Chunk, chunk_x: i32, chunk_y: i32, start: Pos, target: Pos) {
        let mut curr = start;
        chunk.set_tile(curr.x, curr.y, Tile::floor());

        while curr != target {
            // Determine direction to target
            let dx = target.x - curr.x;
            let dy = target.y - curr.y;

            let move_x = dx.signum();
            let move_y = dy.signum();

            // Randomly choose whether to move along X or Y, with bias towards the larger distance
            // But also allow "mistakes" (moving perpendicular) to make it organic.

            let r = rand::gen_range(0, 100);

            let next_pos = if r < 70 {
                // Move towards target
                if dx.abs() > dy.abs() {
                    Pos::new(curr.x + move_x, curr.y)
                } else {
                    Pos::new(curr.x, curr.y + move_y)
                }
            } else if r < 85 {
                // Move towards target (secondary axis)
                if dx.abs() > dy.abs() {
                    Pos::new(curr.x, curr.y + move_y)
                } else {
                     Pos::new(curr.x + move_x, curr.y)
                }
            } else {
                // Move perpendicular / random wander
                 let wander_dir = rand::gen_range(0, 4);
                 match wander_dir {
                     0 => Pos::new(curr.x, curr.y - 1),
                     1 => Pos::new(curr.x, curr.y + 1),
                     2 => Pos::new(curr.x - 1, curr.y),
                     3 => Pos::new(curr.x + 1, curr.y),
                     _ => curr
                 }
            };

            // Clamp and Check bounds
            if next_pos.x >= 0 && next_pos.x < CHUNK_SIZE && next_pos.y >= 0 && next_pos.y < CHUNK_SIZE {
                // Ensure we don't violate boundary consistency!
                if self.is_move_allowed(chunk, chunk_x, chunk_y, next_pos.x, next_pos.y) {
                    curr = next_pos;
                    chunk.set_tile(curr.x, curr.y, Tile::floor());
                } else {
                    // blocked by wall neighbor, force valid move towards target to avoid infinite loop
                     if dx.abs() > dy.abs() {
                        curr.x += move_x;
                    } else {
                        curr.y += move_y;
                    }
                    // If even the forced move is blocked, we just break/stop or accept it?
                    // If we are forcing a path, we usually want to succeed.
                    // But if strict boundary is required, we can't overwrite the wall.
                    // The 'target' should be valid (an empty edge).
                    // So we should eventually reach it.
                    if curr.x >= 0 && curr.x < CHUNK_SIZE && curr.y >= 0 && curr.y < CHUNK_SIZE {
                        if self.is_move_allowed(chunk, chunk_x, chunk_y, curr.x, curr.y) {
                            chunk.set_tile(curr.x, curr.y, Tile::floor());
                        }
                    }
                }
            }

            // Safety break if we get stuck
            if (curr.x - target.x).abs() + (curr.y - target.y).abs() < 1 {
                 chunk.set_tile(target.x, target.y, Tile::floor()); // Ensure target is floor
                 break;
            }
        }
    }
}
