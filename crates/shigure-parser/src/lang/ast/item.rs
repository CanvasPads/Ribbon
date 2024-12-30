use super::TokenLoc;
use serde::{Deserialize, Serialize};

/// A location information for  nodes
#[derive(Eq, PartialEq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Loc {
    pub start: u32,
    pub end: u32,
}

impl From<TokenLoc> for Loc {
    fn from(item: TokenLoc) -> Self {
        Loc {
            start: item.starts_at,
            end: item.starts_at + item.len,
        }
    }
}

/// A node that has [`Loc`] in own member.
pub trait HasLoc {
    fn loc(&self) -> Loc;
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeStructured {
    pub loc: Loc,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeNumberLiteral {
    pub loc: Loc,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeStringLiteral {
    pub loc: Loc,
    pub value: String,
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeArray {
    pub values: Vec<NodeValue>,
}

pub struct NodeBlock {}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum NodeValue {
    Structured(NodeStructured),
    Array(NodeArray),
    StringLiteral(NodeStringLiteral),
    NumberLiteral(NodeNumberLiteral),
    Identifier(NodeIdentifier),
    Block,
}

pub struct NodeViewElement {
    loc: Loc,
}

impl HasLoc for NodeViewElement {
    fn loc(&self) -> Loc {
        self.loc
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeIdentifier {
    pub value: String,
    pub loc: Loc,
}

impl HasLoc for NodeIdentifier {
    fn loc(&self) -> Loc {
        self.loc
    }
}

pub struct NodeParameter {
    pub loc: Loc,
}

impl HasLoc for NodeParameter {
    fn loc(&self) -> Loc {
        self.loc
    }
}

pub struct NodeAssignmentOp {
    pub loc: Loc,
}

impl HasLoc for NodeAssignmentOp {
    fn loc(&self) -> Loc {
        self.loc
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeConst {
    pub name: NodeIdentifier,
    pub loc: Loc,
}

impl HasLoc for NodeConst {
    fn loc(&self) -> Loc {
        self.loc
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeView {
    pub loc: Loc,
}

impl HasLoc for NodeView {
    fn loc(&self) -> Loc {
        self.loc
    }
}

///  nodes that possibly placement in a block
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum NodeScoped {
    Const(NodeConst),
    View(NodeView),
}

impl HasLoc for NodeScoped {
    fn loc(&self) -> Loc {
        match self {
            NodeScoped::Const(i) => i.loc(),
            NodeScoped::View(i) => i.loc(),
        }
    }
}

/// A module node
#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeModule {
    pub loc: Loc,
    pub name: String,
    pub nodes: Vec<NodeScoped>,
}

impl HasLoc for NodeModule {
    fn loc(&self) -> Loc {
        self.loc
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct NodeFile {
    pub loc: Loc,
    pub name: String,
    pub modules: Vec<NodeModule>,
}

impl HasLoc for NodeFile {
    fn loc(&self) -> Loc {
        self.loc
    }
}
