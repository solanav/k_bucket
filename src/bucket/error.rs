use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BucketError {
    BucketFull,
    IncorrectBucket,
    RepeatedLNode,
    LNodeNotFound,
    NodeNotFound,
    IndexError,
}

impl std::error::Error for BucketError {}

impl fmt::Display for BucketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BucketError::BucketFull => write!(f, "This bucket is full."),
            BucketError::IncorrectBucket => write!(f, "The node does not fit."),
            BucketError::RepeatedLNode => write!(f, "There already is a local node."),
            BucketError::LNodeNotFound => write!(f, "Could not find the local node."),
            BucketError::NodeNotFound => write!(f, "Could not find the node."),
            BucketError::IndexError => write!(f, "Index is out of bounds."),
        }
    }
}