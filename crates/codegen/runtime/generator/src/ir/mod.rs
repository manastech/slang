use codegen_language_definition::model::{self, PredefinedLabel};
use indexmap::{IndexMap, IndexSet};
use serde::Serialize;

#[derive(Serialize)]
pub struct ModelWrapper {
    source: Option<IrModel>,
    target: IrModel,
}

impl ModelWrapper {
    pub fn new(name: &str, language: &model::Language) -> Self {
        ModelWrapper {
            source: None,
            target: IrModel::from_language(name, language),
        }
    }

    pub fn from(name: &str, other: ModelWrapper) -> Self {
        let source = other.target;
        let mut target = IrModel::from_model(name, &source);

        for (_, sequence) in &mut target.sequences {
            sequence.can_build_from_source = true;
        }

        ModelWrapper {
            source: Some(source),
            target,
        }
    }

    pub fn remove_type(&mut self, name: &str) -> bool {
        let identifier: model::Identifier = name.into();
        let removed = self.target.sequences.shift_remove(&identifier).is_some()
            || self.target.choices.shift_remove(&identifier).is_some()
            || self.target.repeated.shift_remove(&identifier).is_some()
            || self.target.separated.shift_remove(&identifier).is_some()
            || self.target.unique_terminals.shift_remove(&identifier)
            || self.target.terminals.shift_remove(&identifier);

        for (_, sequence) in &mut self.target.sequences {
            let mut index = 0;
            while index < sequence.fields.len() {
                if sequence.fields[index].r#type == identifier {
                    sequence.fields.remove(index);
                } else {
                    index += 1;
                }
            }
        }

        for (_, choice) in &mut self.target.choices {
            let mut index = 0;
            let mut removed_some = false;
            while index < choice.terminal_types.len() {
                if choice.terminal_types[index] == identifier {
                    choice.terminal_types.remove(index);
                    removed_some = true;
                } else {
                    index += 1;
                }
            }
            index = 0;
            while index < choice.nonterminal_types.len() {
                if choice.nonterminal_types[index] == identifier {
                    choice.nonterminal_types.remove(index);
                    removed_some = true;
                } else {
                    index += 1;
                }
            }
            choice.has_removed_variants = choice.has_removed_variants || removed_some;
        }

        if let Some(repeated_with_item_type) =
            self.target.repeated.iter().find_map(|repeated_entry| {
                if repeated_entry.1.item_type == identifier {
                    Some(repeated_entry.0.clone())
                } else {
                    None
                }
            })
        {
            self.target.repeated.shift_remove(&repeated_with_item_type);
        }

        if let Some(separated_with_item_type) =
            self.target.separated.iter().find_map(|separated_entry| {
                if separated_entry.1.item_type == identifier {
                    Some(separated_entry.0.clone())
                } else {
                    None
                }
            })
        {
            self.target
                .separated
                .shift_remove(&separated_with_item_type);
        }

        removed
    }

    pub fn remove_sequence_field(&mut self, sequence_id: &str, field_label: &str) -> bool {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.target.sequences.get_mut(&identifier) else {
            return false;
        };
        let mut index = 0;
        while index < sequence.fields.len() {
            if sequence.fields[index].label == field_label.into() {
                sequence.fields.remove(index);
                return true;
            }
            index += 1;
        }
        false
    }

    pub fn add_sequence_field(
        &mut self,
        sequence_id: &str,
        field_label: &str,
        field_type: &str,
        is_optional: bool,
    ) -> bool {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.target.sequences.get_mut(&identifier) else {
            return false;
        };
        let is_terminal = self
            .target
            .terminals
            .contains::<model::Identifier>(&field_type.into());
        sequence.fields.push(Field {
            label: field_label.into(),
            r#type: field_type.into(),
            is_terminal,
            is_optional,
        });
        sequence.can_build_from_source = false;
        false
    }
}

#[derive(Default, Serialize)]
pub struct IrModel {
    pub name: String,

    #[serde(skip)]
    terminals: IndexSet<model::Identifier>,

    pub unique_terminals: IndexSet<model::Identifier>,

    pub sequences: IndexMap<model::Identifier, Sequence>,
    pub choices: IndexMap<model::Identifier, Choice>,
    pub repeated: IndexMap<model::Identifier, Repeated>,
    pub separated: IndexMap<model::Identifier, Separated>,
}

#[derive(Clone, Serialize)]
pub struct Sequence {
    pub fields: Vec<Field>,
    pub can_build_from_source: bool,
}

#[derive(Clone, Serialize)]
pub struct Field {
    pub label: model::Identifier,

    /// AST Type of the field
    pub r#type: model::Identifier,

    pub is_terminal: bool,
    pub is_optional: bool,
}

#[derive(Clone, Serialize)]
pub struct Choice {
    pub nonterminal_types: Vec<model::Identifier>,
    pub terminal_types: Vec<model::Identifier>,
    pub has_removed_variants: bool,
}

#[derive(Clone, Serialize)]
pub struct Repeated {
    /// AST Type of the field
    pub item_type: model::Identifier,
    pub is_terminal: bool,
}

#[derive(Clone, Serialize)]
pub struct Separated {
    /// AST Type of the field
    pub item_type: model::Identifier,
    pub is_terminal: bool,
}

impl IrModel {
    pub fn from_language(name: &str, language: &model::Language) -> Self {
        let mut model = Self {
            name: name.to_owned(),

            terminals: IndexSet::new(),
            unique_terminals: IndexSet::new(),

            sequences: IndexMap::new(),
            choices: IndexMap::new(),
            repeated: IndexMap::new(),
            separated: IndexMap::new(),
        };

        // First pass: collect all terminals:
        model.collect_terminals(language);

        // Second pass: use them to build nonterminals:
        model.collect_nonterminals(language);

        model
    }

    pub fn from_model(name: &str, model: &Self) -> Self {
        Self {
            name: name.to_owned(),

            terminals: model.terminals.clone(),
            unique_terminals: model.unique_terminals.clone(),

            sequences: model.sequences.clone(),
            choices: model.choices.clone(),
            repeated: model.repeated.clone(),
            separated: model.separated.clone(),
        }
    }

    fn collect_terminals(&mut self, language: &model::Language) {
        for item in language.items() {
            match item {
                model::Item::Struct { .. }
                | model::Item::Enum { .. }
                | model::Item::Repeated { .. }
                | model::Item::Separated { .. }
                | model::Item::Precedence { .. } => {
                    // These items are nonterminals.
                }
                model::Item::Trivia { item } => {
                    self.terminals.insert(item.name.clone());
                }
                model::Item::Keyword { item } => {
                    self.terminals.insert(item.name.clone());
                    if item.is_unique() {
                        self.unique_terminals.insert(item.name.clone());
                    }
                }
                model::Item::Token { item } => {
                    self.terminals.insert(item.name.clone());
                    if item.is_unique() {
                        self.unique_terminals.insert(item.name.clone());
                    }
                }
                model::Item::Fragment { .. } => {
                    // These items are inlined.
                }
            };
        }
    }

    fn collect_nonterminals(&mut self, language: &model::Language) {
        for item in language.items() {
            match item {
                model::Item::Struct { item } => {
                    self.add_struct_item(item);
                }
                model::Item::Enum { item } => {
                    self.add_enum_item(item);
                }
                model::Item::Repeated { item } => {
                    self.add_repeated_item(item);
                }
                model::Item::Separated { item } => {
                    self.add_separated_item(item);
                }
                model::Item::Precedence { item } => {
                    self.add_precedence_item(item);

                    for expr in &item.precedence_expressions {
                        self.add_precedence_expression(&item.name, expr);
                    }
                }
                model::Item::Trivia { .. }
                | model::Item::Keyword { .. }
                | model::Item::Token { .. } => {
                    // These items are terminals.
                }
                model::Item::Fragment { .. } => {
                    // These items are inlined.
                }
            };
        }
    }

    fn add_struct_item(&mut self, item: &model::StructItem) {
        let parent_type = item.name.clone();
        let fields = self.convert_fields(&item.fields).collect();

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                can_build_from_source: false,
            },
        );
    }

    fn add_enum_item(&mut self, item: &model::EnumItem) {
        let parent_type = item.name.clone();

        let (terminal_types, nonterminal_types): (Vec<_>, Vec<_>) = item
            .variants
            .iter()
            .map(|variant| variant.reference.clone())
            .partition(|reference| self.terminals.contains(reference));

        self.choices.insert(
            parent_type,
            Choice {
                nonterminal_types,
                terminal_types,
                has_removed_variants: false,
            },
        );
    }

    fn add_repeated_item(&mut self, item: &model::RepeatedItem) {
        let parent_type = item.name.clone();

        self.repeated.insert(
            parent_type,
            Repeated {
                item_type: item.reference.clone(),
                is_terminal: self.terminals.contains(&item.reference),
            },
        );
    }

    fn add_separated_item(&mut self, item: &model::SeparatedItem) {
        let parent_type = item.name.clone();

        self.separated.insert(
            parent_type,
            Separated {
                item_type: item.reference.clone(),
                is_terminal: self.terminals.contains(&item.reference),
            },
        );
    }

    fn add_precedence_item(&mut self, item: &model::PrecedenceItem) {
        let parent_type = item.name.clone();

        let precedence_expressions = item
            .precedence_expressions
            .iter()
            .map(|expression| expression.name.clone());

        let primary_expressions = item
            .primary_expressions
            .iter()
            .map(|expression| expression.reference.clone());

        let (terminal_types, nonterminal_types): (Vec<_>, Vec<_>) = precedence_expressions
            .chain(primary_expressions)
            .partition(|reference| self.terminals.contains(reference));

        self.choices.insert(
            parent_type,
            Choice {
                nonterminal_types,
                terminal_types,
                has_removed_variants: false,
            },
        );
    }

    fn add_precedence_expression(
        &mut self,
        base_name: &model::Identifier,
        expression: &model::PrecedenceExpression,
    ) {
        let parent_type = expression.name.clone();

        // All operators should have the same structure (validated at compile-time),
        // So let's pick up the first one to generate the types:
        let operator = &expression.operators[0];
        let mut fields = vec![];

        let operand = |label: PredefinedLabel| Field {
            label: label.as_ref().into(),
            r#type: base_name.clone(),
            is_terminal: false,
            is_optional: false,
        };

        match operator.model {
            model::OperatorModel::Prefix => {
                fields.extend(self.convert_fields(&operator.fields));
                fields.push(operand(PredefinedLabel::Operand));
            }
            model::OperatorModel::Postfix => {
                fields.push(operand(PredefinedLabel::Operand));
                fields.extend(self.convert_fields(&operator.fields));
            }
            model::OperatorModel::BinaryLeftAssociative
            | model::OperatorModel::BinaryRightAssociative => {
                fields.push(operand(PredefinedLabel::LeftOperand));
                fields.extend(self.convert_fields(&operator.fields));
                fields.push(operand(PredefinedLabel::RightOperand));
            }
        };

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                can_build_from_source: false,
            },
        );
    }

    fn convert_fields<'a>(
        &'a self,
        fields: &'a IndexMap<model::Identifier, model::Field>,
    ) -> impl Iterator<Item = Field> + 'a {
        fields.iter().map(|(label, field)| {
            let (reference, is_optional) = match field {
                model::Field::Required { reference } => (reference, false),
                model::Field::Optional {
                    reference,
                    enabled: _,
                } => (reference, true),
            };

            Field {
                label: label.clone(),
                r#type: reference.clone(),
                is_terminal: self.terminals.contains(reference),
                is_optional,
            }
        })
    }
}
