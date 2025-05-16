use std::cell::RefCell;
use std::collections::HashMap;

use crate::cst::NodeId;

pub struct Binder {
    pub definitions: HashMap<NodeId, Definition>,
    pub references: HashMap<NodeId, Reference>,
    scopes: HashMap<ScopeId, RefCell<Scope>>,
    nodes_to_scopes: HashMap<NodeId, ScopeId>,
}

impl Binder {
    fn new() -> Self {
        Self {
            definitions: HashMap::new(),
            references: HashMap::new(),
            scopes: HashMap::new(),
            nodes_to_scopes: HashMap::new(),
        }
    }
}

impl Default for Binder {
    fn default() -> Self {
        Self::new()
    }
}

impl Binder {
    pub(crate) fn insert_scope_at(&mut self, node_id: NodeId, parent: Option<ScopeId>) -> ScopeId {
        let scope_id = node_id.into();
        let scope = RefCell::new(Scope {
            parent,
            definitions: HashMap::new(),
        });
        self.scopes.insert(scope_id, scope);
        self.nodes_to_scopes.insert(node_id, scope_id);
        scope_id
    }

    pub(crate) fn link_node_to_scope(&mut self, node_id: NodeId, scope_id: ScopeId) {
        if !self.scopes.contains_key(&scope_id) {
            unreachable!("scope {scope_id:?} doesn't exist");
        }
        self.nodes_to_scopes.insert(node_id, scope_id);
    }

    pub(crate) fn resolve_at(&self, node_id: NodeId, name: &str) -> Option<NodeId> {
        let Some(scope_id) = self.nodes_to_scopes.get(&node_id) else {
            unreachable!("scope for node {node_id:?} is unknown");
        };
        self.resolve_at_scope(*scope_id, name)
    }

    fn resolve_at_scope(&self, scope_id: ScopeId, name: &str) -> Option<NodeId> {
        let Some(scope) = self.scopes.get(&scope_id) else {
            unreachable!("scope {scope_id:?} doesn't exist");
        };
        let scope = scope.borrow();
        if let Some(definition) = scope.definitions.get(name) {
            return Some(*definition);
        }
        if let Some(parent_scope) = scope.parent {
            self.resolve_at_scope(parent_scope, name)
        } else {
            None
        }
    }

    pub(crate) fn insert_definition_at_scope(
        &mut self,
        scope_id: ScopeId,
        name_node_id: NodeId,
        name: String,
        definiens_node_id: NodeId,
    ) {
        let Some(scope) = self.scopes.get(&scope_id) else {
            unreachable!("scope {scope_id:?} doesn't exist");
        };
        let mut scope = scope.borrow_mut();
        scope.definitions.insert(name.clone(), definiens_node_id);

        let definition = Definition {
            name,
            name_node_id,
            definiens_node_id,
        };
        self.definitions.insert(definiens_node_id, definition);
    }

    pub(crate) fn insert_reference_scoped_at(
        &mut self,
        scope_node_id: NodeId,
        node_id: NodeId,
        name: String,
    ) {
        let definition_id = self.resolve_at(scope_node_id, &name);
        let reference = Reference {
            name,
            node_id,
            definition_id,
        };
        self.references.insert(node_id, reference);
    }

    // TODO: temporary function
    pub(crate) fn insert_unresolved_reference(&mut self, node_id: NodeId, name: String) {
        let reference = Reference {
            name,
            node_id,
            definition_id: None,
        };
        self.references.insert(node_id, reference);
    }
}

pub struct Definition {
    pub name: String,
    pub name_node_id: NodeId,
    pub definiens_node_id: NodeId,
}

pub struct Reference {
    pub name: String,
    pub node_id: NodeId,
    pub definition_id: Option<NodeId>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ScopeId(usize);

impl From<NodeId> for ScopeId {
    fn from(value: NodeId) -> Self {
        Self(value.into())
    }
}

struct Scope {
    parent: Option<ScopeId>,
    definitions: HashMap<String, NodeId>,
}
