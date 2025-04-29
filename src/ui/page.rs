// src/ui/page.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Dashboard,
    GroupDetail(usize), // selected group index
}
