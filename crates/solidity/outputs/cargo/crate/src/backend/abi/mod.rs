use crate::cst::NodeId;

#[derive(Debug)]
pub struct ContractAbi {
    pub node_id: NodeId,
    pub name: String,
    pub file_id: String,
    pub functions: Vec<FunctionAbi>,
    pub storage_layout: Vec<Slot>,
}

#[derive(Debug)]
pub struct FunctionAbi {
    pub node_id: NodeId,
    pub name: Option<String>,
    pub kind: FunctionAbiType,
    pub inputs: Vec<FunctionInputOutput>,
    pub outputs: Vec<FunctionInputOutput>,
    pub state_mutability: FunctionAbiMutability,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FunctionAbiType {
    Function,
    Constructor,
    Receive,
    Fallback,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FunctionAbiMutability {
    NonPayable,
    Payable,
    View,
    Pure,
}

#[derive(Debug)]
pub struct FunctionInputOutput {
    pub node_id: Option<NodeId>, // will be `None` if the function is a generated getter
    pub name: Option<String>,
    pub r#type: String,
}

#[derive(Debug)]
pub struct Slot {
    pub node_id: NodeId,
    pub label: String,
    pub slot: usize,
    pub offset: usize,
    pub r#type: String,
}
