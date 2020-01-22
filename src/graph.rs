pub mod graph {
    
    use std::collections::HashMap;
    
    pub struct Graph {
        nodes: HashMap<i32, GraphNode>
    }

    pub struct GraphNode {
        id: i32,
        adjacents: Vec<i32>
    }

    
}