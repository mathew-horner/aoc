#![feature(iter_collect_into)]
#![feature(linked_list_cursors)]
#![feature(pattern)]

pub mod date;
pub mod input;
pub mod solutions;

// Re-export for convenient references from solutions (`crate::Input`)
pub use input::Input;
