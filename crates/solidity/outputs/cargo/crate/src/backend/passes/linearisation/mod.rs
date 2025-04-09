use std::collections::HashMap;

use crate::backend::{
    ast,
    types::{ContractType, InterfaceType, TypeDefinition, TypeRegistry},
};

mod c3;

pub struct Pass {
    pub linearisations: HashMap<usize, Vec<usize>>,
    pub types: TypeRegistry,
}

impl Pass {
    pub fn new(types: TypeRegistry) -> Self {
        Self {
            types,
            linearisations: HashMap::new(),
        }
    }

    fn find_contract_bases_recursively(&self, node_id: usize) -> HashMap<usize, Vec<usize>> {
        let mut result = HashMap::new();
        let mut queue = vec![node_id];
        while let Some(node_id) = queue.pop() {
            if result.contains_key(&node_id) {
                continue;
            }
            let Some(type_definition) = self.types.find_definition(node_id) else {
                panic!("Unable to resolve type definition for node {node_id}");
            };
            let (TypeDefinition::Interface(InterfaceType { base_types, .. })
            | TypeDefinition::Contract(ContractType { base_types, .. })) = type_definition
            else {
                panic!(
                    "Found {type_definition:?} resolving parents of node {node_id} \
                        and it's not a contract or an interface"
                );
            };
            // Resolve parents recursively
            queue.extend(base_types.iter());

            // Solidity uses a modified version of the traditional C3 algorithm
            // where the order of parents is reversed, so we provide them in the
            // inverse order
            let mut parents = base_types.clone();
            parents.reverse();
            result.insert(node_id, parents);
        }
        result
    }
}

impl ast::visitor::Visitor for Pass {
    fn leave_contract_definition(&mut self, target: &ast::ContractDefinition) {
        let node_id = target.node_id;
        let parents = self.find_contract_bases_recursively(node_id);
        if let Some(linearisation) = c3::linearise(&node_id, &parents) {
            self.linearisations.insert(node_id, linearisation);
        } else {
            panic!(
                "Failed to linearise contract {name}",
                name = target.name.unparse()
            );
        }
    }
}
