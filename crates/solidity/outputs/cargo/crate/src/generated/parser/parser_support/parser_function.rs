// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use crate::cst::{Edge, Node, TerminalKind, TerminalKindExtensions, TextIndex};
use crate::parser::lexer::Lexer;
use crate::parser::parser_support::context::ParserContext;
use crate::parser::parser_support::parser_result::{
    IncompleteMatch, Match, ParserResult, SkippedUntil,
};
use crate::parser::{ParseError, ParseOutput};

pub trait ParserFunction<P>
where
    Self: Fn(&P, &mut ParserContext<'_>) -> ParserResult,
{
    fn parse(&self, parser: &P, input: &str) -> ParseOutput;
}

impl<P, F> ParserFunction<P> for F
where
    P: Lexer,
    F: Fn(&P, &mut ParserContext<'_>) -> ParserResult,
{
    #[allow(clippy::too_many_lines)]
    fn parse(&self, parser: &P, input: &str) -> ParseOutput {
        let mut stream = ParserContext::new(input);
        let mut result = self(parser, &mut stream);

        // For a succesful/recovered parse, collect any remaining trivia as part of the parse result
        if let ParserResult::Match(r#match) = &mut result {
            let [topmost] = r#match.nodes.as_mut_slice() else {
                unreachable!(
                    "Match at the top level of a parse does not have exactly one Nonterminal node"
                )
            };

            let eof_trivia = match Lexer::leading_trivia(parser, &mut stream) {
                ParserResult::Match(eof_trivia) if !eof_trivia.nodes.is_empty() => {
                    Some(eof_trivia.nodes)
                }
                _ => None,
            };

            if let (Node::Nonterminal(nonterminal), Some(eof_trivia)) =
                (&mut topmost.node, eof_trivia)
            {
                let mut new_children = nonterminal.children.clone();
                new_children.extend(eof_trivia);

                topmost.node = Node::nonterminal(nonterminal.kind, new_children);
            }
        }

        let is_incomplete = matches!(result, ParserResult::IncompleteMatch(_));
        let is_recovering = matches!(result, ParserResult::SkippedUntil(_));

        match result {
            ParserResult::PrattOperatorMatch(..) => unreachable!("PrattOperatorMatch is internal"),

            ParserResult::NoMatch(mut no_match) => {
                let (kind, start) = if input.is_empty() {
                    (TerminalKind::MISSING, TextIndex::ZERO)
                } else {
                    let mut ti = TextIndex::ZERO;
                    no_match.nodes.iter().for_each(|edge| {
                        if let Node::Terminal(terminal) = &edge.node {
                            if terminal.kind.is_valid() {
                                ti.advance_str(terminal.text.as_str());
                            }
                        }
                    });
                    (TerminalKind::UNRECOGNIZED, ti)
                };

                let node = Node::terminal(kind, input[start.utf8..].to_string());
                let tree = if no_match.kind.is_none() || start.utf8 == 0 {
                    node
                } else {
                    no_match.nodes.push(Edge::anonymous(node));
                    Node::nonterminal(no_match.kind.unwrap(), no_match.nodes)
                };
                ParseOutput {
                    parse_tree: tree,
                    errors: vec![ParseError::new(
                        start..input.into(),
                        no_match.expected_terminals,
                    )],
                }
            }
            some_match => {
                let (nodes, expected_terminals) = match some_match {
                    ParserResult::PrattOperatorMatch(..) | ParserResult::NoMatch(..) => {
                        unreachable!("Handled above")
                    }
                    ParserResult::Match(Match {
                        nodes,
                        expected_terminals,
                    })
                    | ParserResult::IncompleteMatch(IncompleteMatch {
                        nodes,
                        expected_terminals,
                    }) => (nodes, expected_terminals),

                    ParserResult::SkippedUntil(SkippedUntil {
                        nodes, expected, ..
                    }) => (nodes, vec![expected]),
                };

                let topmost_node = match &nodes[..] {
                    [Edge { node: Node::Nonterminal(nonterminal), ..} ] => Rc::clone(nonterminal),
                    [_] => unreachable!(
                        "(Incomplete)Match at the top level of a parser is not a Nonterminal node"
                    ),
                    _ => unreachable!(
                        "(Incomplete)Match at the top level of a parser does not have exactly one node"
                    ),
                };

                let start = stream.position();

                // Mark the rest of the unconsumed stream as skipped and report an error
                // NOTE: IncompleteMatch internally consumes the stream when picked via choice,
                // so needs a separate check here.
                if start < input.len() || is_incomplete || is_recovering {
                    let start = if is_recovering {
                        topmost_node.text_len.utf8
                    } else {
                        start
                    };
                    let kind = if input[start..].is_empty() {
                        TerminalKind::MISSING
                    } else {
                        TerminalKind::UNRECOGNIZED
                    };
                    let skipped_node = Node::terminal(kind, input[start..].to_string());
                    let mut new_children = topmost_node.children.clone();
                    new_children.push(Edge::anonymous(skipped_node));

                    let start_index = stream.text_index_at(start);
                    let mut errors = stream.into_errors();
                    errors.push(ParseError::new(
                        start_index..input.into(),
                        expected_terminals,
                    ));

                    ParseOutput {
                        parse_tree: Node::nonterminal(topmost_node.kind, new_children),
                        errors,
                    }
                } else {
                    let parse_tree = Node::Nonterminal(topmost_node);
                    let errors = stream.into_errors();

                    // Sanity check: Make sure that succesful parse is equivalent to not having any invalid nodes
                    debug_assert_eq!(
                        errors.is_empty(),
                        parse_tree
                            .cursor_with_offset(TextIndex::ZERO)
                            .all(|node| node
                                .as_terminal()
                                .filter(|tok| !tok.kind.is_valid())
                                .is_none())
                    );

                    ParseOutput { parse_tree, errors }
                }
            }
        }
    }
}
