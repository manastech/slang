use crate::cst::NodeId;

pub struct ContractAbi {
    pub node_id: NodeId,
    pub name: String,
    pub file_id: String,
    pub functions: Vec<FunctionAbi>,
    pub storage_layout: Vec<Slot>,
}

pub struct FunctionAbi {
    pub node_id: NodeId,
    pub name: Option<String>,
    pub kind: FunctionAbiType,
    pub inputs: Vec<FunctionInputOutput>,
    pub outputs: Vec<FunctionInputOutput>,
    pub state_mutability: FunctionAbiMutability,
}

pub enum FunctionAbiType {
    Function,
    Constructor,
    Receive,
    Fallback,
}

pub enum FunctionAbiMutability {
    NonPayable,
    Payable,
    View,
    Pure,
}

pub struct FunctionInputOutput {
    pub node_id: NodeId,
    pub name: Option<String>,
    pub r#type: String,
    pub internal_type: String,
}

pub struct Slot {
    pub node_id: NodeId,
    pub label: String,
    pub slot: usize,
    pub offset: usize,
    pub r#type: String,
}
