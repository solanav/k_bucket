/// Wrapper to store many buckets
pub mod bucket_list;

use crate::id::Id;
use crate::node::Node;
use std::fmt;
use crate::error::BucketError;

pub struct Bucket {
    node_list: Vec<Node>,
    start_id: Id,
    end_id: Id,
    max_nodes: usize,
}

impl Bucket {
    pub fn new(max_nodes: usize, start_id: Id, end_id: Id) -> Self {
        Self {
            node_list: Vec::new(),
            start_id,
            end_id,
            max_nodes,
        }
    }

    /// Add a new node, can fail if the bucket is full, if the node should not
    /// be inside this bucket and if the node is Local but there already is a
    /// local node in this bucket.
    pub fn add_node(&mut self, node: Node) -> Result<(), BucketError> {
        if self.node_list.len() >= self.max_nodes {
            return Err(BucketError::BucketFull);
        }

        if !self.fits(&node.id()) {
            return Err(BucketError::IncorrectBucket);
        }

        if self.contains_local() && node.is_local() {
            return Err(BucketError::RepeatedLNode);
        }

        self.node_list.push(node);
        Ok(())
    }

    /// Remove a node by its ID
    pub fn rm_node(&mut self, id: Id) {
        let mut to_remove = Vec::new();

        for (i, node) in self.node_list.iter().enumerate() {
            if node.id() == id {
                to_remove.push(i);
            }
        }

        for i in to_remove {
            self.node_list.remove(i);
        }
    }

    /// Check if the bucket contains the node that represents us
    pub fn contains_local(&self) -> bool {
        for node in self.node_list.iter() {
            if node.is_local() {
                return true;
            }
        }

        false
    }

    /// Divide bucket and split the ID space between the two.
    /// It also moves the nodes to the new bucket if necessary.
    pub fn divide(&mut self) -> Option<Self> {
        if self.node_list.len() >= self.max_nodes {
            println!("Too many nodes");
            return None;
        }

        if self.contains_local() == false {
            println!("No local");
            return None;
        }

        // Update the end_id
        let end_id = self.end_id;
        self.end_id = self.end_id.half();

        let mut new_bucket = Bucket::new(self.max_nodes, self.end_id + 1, end_id);

        // Move nodes to new bucket and let add_node check for errors
        for _ in 0..self.node_list.len() {
            let node = self.node_list.pop();

            match node {
                Some(n) => {
                    let id = n.id();
                    if let Ok(_) = new_bucket.add_node(n) {
                        self.rm_node(id);
                    }
                },
                None => break,
            }
        }

        Some(new_bucket)
    }

    /// Check if a node fits inside the bucket (in terms of ID)
    pub fn fits(&self, id: &Id) -> bool {
        *id > self.start_id && *id < self.end_id
    }

    /// Get list of nodes inside a bucket
    pub fn nodes(&mut self) -> Vec<&mut Node> {
        let mut node_list = Vec::new();
        for node in self.node_list.iter_mut() {
            node_list.push(node);
        }
        node_list
    }
}

impl fmt::Debug for Bucket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = format!(
            "{:?}\t{:?}\n\t{:?}",
            self.contains_local(),
            self.start_id,
            self.end_id
        );

        for node in self.node_list.iter() {
            output = format!("{}\n\t{:?}", output, node);
        }

        write!(f, "{}\n", output)
    }
}
