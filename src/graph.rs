pub mod graph {
    
    use std::collections::HashMap;
    
    pub struct Graph {
        nodes: HashMap<u32, GraphNode>
    }

    impl Graph {
      fn add_node(&mut self) -> u32 {
        (*self).nodes.keys().fold(0, move |curr, potential| std::cmp::max(curr, *potential))
      }
    }

    struct GraphNode {
        id: u32,
        adjacents: Vec<u32>
    }
}