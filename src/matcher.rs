use std::iter::zip;

use crate::{
    ast::{ASTNode, ASTNodeType},
    env::Environment,
    exec,
    object::{ExtensionList, Object},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Match {
    Match(Vec<(String, Object)>),
    NotAMatch,
}

pub fn match_call(patterns: &[ASTNode], args: &[Object]) -> Match {
    match_list(patterns, args)
}

fn join(map1: Match, map2: Match) -> Match {
    match (map1, map2) {
        (Match::Match(v1), Match::Match(v2)) => {
            for (key1, val1) in &v1 {
                for (key2, val2) in &v2 {
                    if key1 == key2 && val1 != val2 {
                        return Match::NotAMatch;
                    }
                }
            }

            let mut map = vec![];
            map.extend(v1);
            map.extend(v2);

            Match::Match(map)
        }
        _ => Match::NotAMatch,
    }
}

fn match_list(patterns: &[ASTNode], vals: &[Object]) -> Match {
    if patterns.len() != vals.len() {
        Match::NotAMatch
    } else {
        zip(patterns, vals)
            .map(|(pattern, val)| match_(pattern, val))
            .fold(empty_match(), join)
    }
}

fn match_(pattern: &ASTNode, val: &Object) -> Match {
    match &pattern._type {
        ASTNodeType::Wildcard => empty_match(),
        ASTNodeType::Symbol(s) => single_match(s, val),
        ASTNodeType::ExtensionList(l) => match_extension_list(l, val),
        ASTNodeType::Prepend(first, most) => match_prefix_crop(first, most, val),
        _ => match_constant(pattern, val),
    }
}

fn single_match(name: &str, val: &Object) -> Match {
    Match::Match(vec![(name.to_string(), val.clone())])
}

fn empty_match() -> Match {
    Match::Match(vec![])
}

fn match_extension_list(pattern: &[ASTNode], val: &Object) -> Match {
    match val {
        Object::ExtensionList(ExtensionList { list: al }) => match_list(pattern, al),
        _ => Match::NotAMatch,
    }
}

fn match_prefix_crop(first: &ASTNode, most: &ASTNode, val: &Object) -> Match {
    match val {
        Object::ExtensionList(ExtensionList { list }) if !list.is_empty() => {
            let first_match = match_(first, &list[0]);

            let last_list = Object::ExtensionList(ExtensionList::from(list[1..].to_owned()));
            let last_match = match_(most, &last_list);

            join(first_match, last_match)
        }
        _ => Match::NotAMatch,
    }
}

fn match_constant(pattern: &ASTNode, val: &Object) -> Match {
    if exec(pattern, &mut Environment::default()).unwrap() == *val {
        empty_match()
    } else {
        Match::NotAMatch
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{_dummy_pos, _extension_list, _prepend, _symbol},
        object::Integer,
    };

    use super::*;

    #[test]
    fn two_args() {
        let patterns = [
            _extension_list(vec![_symbol("a", _dummy_pos())], _dummy_pos()),
            _extension_list(vec![_symbol("b", _dummy_pos())], _dummy_pos()),
        ];

        let args = [
            Object::ExtensionList(ExtensionList {
                list: vec![Object::Integer(Integer::from(1))],
            }),
            Object::ExtensionList(ExtensionList {
                list: vec![Object::Integer(Integer::from(2))],
            }),
        ];

        assert_eq!(
            match_list(&patterns, &args),
            Match::Match(vec![
                (String::from("a"), Object::Integer(Integer::from(1))),
                (String::from("b"), Object::Integer(Integer::from(2)))
            ])
        );
    }

    #[test]
    fn list_prefix() {
        let pattern = _prepend(
            _symbol("first", _dummy_pos()),
            _symbol("most", _dummy_pos()),
            _dummy_pos(),
        );

        let value =
            Object::ExtensionList(ExtensionList::from(vec![Object::Integer(Integer::from(4))]));

        assert_eq!(
            match_(&pattern, &value),
            Match::Match(vec![
                (String::from("first"), Object::Integer(Integer::from(4))),
                (
                    String::from("most"),
                    Object::ExtensionList(ExtensionList::from(vec![]))
                ),
            ]),
        );
    }

    #[test]
    fn unmatch_different_value() {
        let patterns = [_symbol("a", _dummy_pos()), _symbol("a", _dummy_pos())];

        let values = [
            Object::Integer(Integer::from(1)),
            Object::Integer(Integer::from(2)),
        ];

        assert_eq!(match_call(&patterns, &values), Match::NotAMatch);
    }
}
