use crate::error::{Error, Position};
use crate::object::{self, Callable, ComprehensionSet, ExtensionList, Function};

use crate::ast::{ASTNode, ASTNodeType_, InfixOperator, PrefixOperator};
use crate::env::Environment;
use crate::object::{
    Bool, Char, DefinedFunction, ExtensionSet, Integer, MyString, Object, Symbol, Tuple,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EvalError {
    MissingFunctionArguments,
    NonCallableObject,
    NonExistentOperation,
    NonIterableObject,
    NonPrependableObject,
}

fn truthy(val: Object) -> bool {
    match val {
        Object::Boolean(boolean) => boolean.value(),
        _ => false,
    }
}

pub fn list(l: &[ASTNode], env: &mut Environment) -> Result<Vec<Object>, Error> {
    l.iter().map(|node| exec(node, env)).collect()
}

fn function(params: &[String], proc: &[ASTNode]) -> Result<Object, Error> {
    Ok(Object::Function(object::Function::DefinedFunction(
        DefinedFunction::new(params.to_owned(), proc.to_owned()),
    )))
}

pub fn exec(node: &ASTNode, env: &mut Environment) -> Result<Object, Error> {
    match &node._type {
        ASTNodeType_::Symbol(str) => symbol(str, env),
        ASTNodeType_::ExtensionSet(l) => extension_set(l, env),
        ASTNodeType_::Integer(str) => integer(str),
        ASTNodeType_::Function(params, proc) => function(params, proc),
        ASTNodeType_::Infix(op, lhs, rhs) => {
            infix(*op, exec(lhs, env)?, exec(rhs, env)?, node.position)
        }
        ASTNodeType_::Let(ident, params, value) if params.is_empty() => let_(ident, value, env),
        ASTNodeType_::Let(ident, args, value) => let_function(ident, args, value, env),
        ASTNodeType_::Boolean(val) => boolean(*val),
        ASTNodeType_::Call(func_node, args) => call(func_node, args, env, node.position),
        ASTNodeType_::Char(chr) => char(*chr),
        ASTNodeType_::ComprehensionSet(value, prop) => comprehension_set(value, prop),
        ASTNodeType_::If(cond, first, second) => if_(exec(cond, env)?, first, second, env),
        ASTNodeType_::Prefix(op, node) => prefix(*op, exec(node, env)?, node.position),
        ASTNodeType_::Signature(_, _) => todo!(),
        ASTNodeType_::String(str) => string(str),
        ASTNodeType_::Tuple(l) => tuple(l, env),
        ASTNodeType_::For(symbol, iterable, proc) => for_(symbol, iterable, proc, env),
        ASTNodeType_::ExtensionList(l) => extension_list(l, env),
        ASTNodeType_::ComprehensionList(transform, prop) => {
            comprehension_list(transform, prop, env)
        }
        ASTNodeType_::Wildcard => todo!(),
        ASTNodeType_::Prepend(first, most) => prepend(exec(first, env)?, most, env),
    }
}

fn prepend(first: Object, most: &ASTNode, env: &mut Environment) -> Result<Object, Error> {
    if let Object::ExtensionList(lst) = exec(most, env)? {
        let mut res = vec![first];
        res.append(&mut lst.list.clone());

        Ok(Object::ExtensionList(res.into()))
    } else {
        Err(Error(EvalError::NonPrependableObject.into(), most.position))
    }
}

fn extension_list(l: &[ASTNode], env: &mut Environment) -> Result<Object, Error> {
    list(l, env).map(|lst| Object::ExtensionList(ExtensionList::from(lst)))
}

fn tuple(l: &[ASTNode], env: &mut Environment) -> Result<Object, Error> {
    list(l, env).map(|lst| Object::Tuple(Tuple::from(lst)))
}

fn string(str: &str) -> Result<Object, Error> {
    Ok(Object::String(MyString::from(str)))
}

fn comprehension_set(value: &ASTNode, prop: &ASTNode) -> Result<Object, Error> {
    Ok(Object::ComprehensionSet(ComprehensionSet::from((
        value.clone(),
        prop.clone(),
    ))))
}

fn char(chr: char) -> Result<Object, Error> {
    Ok(Object::Char(Char::from(chr)))
}

fn boolean(val: bool) -> Result<Object, Error> {
    Ok(Object::Boolean(Bool::from(val)))
}

fn let_(ident: &ASTNode, value: &ASTNode, env: &mut Environment) -> Result<Object, Error> {
    match &ident._type {
        ASTNodeType_::Symbol(name) => exec_and_set(value, name, env),
        ASTNodeType_::Signature(ident, None) => match &ident._type {
            ASTNodeType_::Symbol(name) => exec_and_set(value, name, env),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn integer(str: &str) -> Result<Object, Error> {
    Ok(Object::Integer(Integer::from(str)))
}

fn extension_set(l: &[ASTNode], env: &mut Environment) -> Result<Object, Error> {
    list(l, env).map(|lst| Object::ExtensionSet(ExtensionSet::from(lst)))
}

fn symbol(str: &str, env: &mut Environment) -> Result<Object, Error> {
    match env.get(str) {
        Some(obj) => Ok(obj.clone()),
        None => Ok(Object::Symbol(Symbol::from(str))),
    }
}

fn comprehension_list(
    transform: &ASTNode,
    prop: &ASTNode,
    env: &mut Environment,
) -> Result<Object, Error> {
    let (symbol, iterator) = match &prop._type {
        ASTNodeType_::Infix(InfixOperator::In, lhs, rhs) => match (&lhs._type, &rhs._type) {
            (ASTNodeType_::Symbol(ident), ASTNodeType_::ExtensionList(l)) => (ident, list(l, env)?),
            _ => todo!(),
        },
        _ => todo!(),
    };

    let mut new_list = vec![];
    env.push_scope();

    for val in iterator {
        env.set(symbol, val);
        new_list.push(exec(transform, env)?);
    }

    Ok(Object::ExtensionList(ExtensionList::from(new_list)))
}

fn let_function(
    ident: &ASTNode,
    args: &[ASTNode],
    value: &ASTNode,
    env: &mut Environment,
) -> Result<Object, Error> {
    let name = match &ident._type {
        ASTNodeType_::Symbol(name) => name,
        _ => todo!(),
    };

    let function: &mut DefinedFunction = match env.get(name) {
        None => {
            env.set(
                name,
                Object::Function(Function::DefinedFunction(DefinedFunction::default())),
            );

            match env.get(name) {
                Some(Object::Function(Function::DefinedFunction(f))) => f,
                _ => todo!(),
            }
        }
        Some(Object::Function(Function::DefinedFunction(f))) => f,
        _ => todo!(),
    };

    function.add_pattern(args, value);

    Ok(Object::Function(Function::DefinedFunction(
        function.clone(),
    )))
}

fn exec_and_set(node: &ASTNode, name: &str, env: &mut Environment) -> Result<Object, Error> {
    let val = exec(node, env)?;
    env.set(name, val.clone());
    Ok(val)
}

fn if_(
    cond: Object,
    first: &ASTNode,
    second: &ASTNode,
    env: &mut Environment,
) -> Result<Object, Error> {
    if truthy(cond) {
        exec(first, env)
    } else {
        exec(second, env)
    }
}

fn for_(
    symbol: &str,
    iterable: &ASTNode,
    proc: &[ASTNode],
    env: &mut Environment,
) -> Result<Object, Error> {
    let obj = exec(iterable, env)?;

    let iter = match &obj {
        Object::ExtensionSet(set) => Ok(set.list()),
        _ => Err(Error(
            EvalError::NonIterableObject.into(),
            iterable.position,
        )),
    }?;

    env.push_scope();

    for val in iter {
        env.set(symbol, val.clone());

        for step in proc {
            exec(step, env)?;
        }
    }

    env.pop_scope();

    Ok(Object::empty_tuple())
}

fn call(
    func_node: &ASTNode,
    args: &[ASTNode],
    env: &mut Environment,
    call_pos: Position,
) -> Result<Object, Error> {
    let func = exec(func_node, env)?;
    if let Object::Function(ref f) = func {
        if args.len() < f.param_number() {
            return Err(Error(EvalError::MissingFunctionArguments.into(), call_pos));
        }
    }

    let mut func_args = vec![];
    for arg in args {
        let func_arg = exec(arg, env)?;
        func_args.push(func_arg);
    }

    match func {
        Object::Function(f) => f.call(&func_args, env),
        _ => Err(Error(
            EvalError::NonCallableObject.into(),
            func_node.position,
        )),
    }
}

fn infix(
    op: InfixOperator,
    lhs: Object,
    rhs: Object,
    infix_pos: Position,
) -> Result<Object, Error> {
    let res = match op {
        InfixOperator::BitwiseAnd => lhs.bitwise_and(rhs),
        InfixOperator::BitwiseXor => lhs.bitwise_xor(rhs),
        InfixOperator::Call => todo!(),
        InfixOperator::Correspondence => todo!(),
        InfixOperator::Division => lhs.over(rhs),
        InfixOperator::Equality => lhs.equality(rhs),
        InfixOperator::Exponentiation => lhs.pow(rhs),
        InfixOperator::Greater => lhs.greater(rhs),
        InfixOperator::GreaterEqual => lhs.greater_equal(rhs),
        InfixOperator::In => rhs.contains(lhs),
        InfixOperator::LeftShift => lhs.left_shift(rhs),
        InfixOperator::Less => lhs.less(rhs),
        InfixOperator::LessEqual => lhs.less_equal(rhs),
        InfixOperator::LogicAnd => lhs.logic_and(rhs),
        InfixOperator::Or => lhs.or(rhs),
        InfixOperator::Mod => lhs.modulo(rhs),
        InfixOperator::NotEquality => lhs.neq(rhs),
        InfixOperator::Product => lhs.product(rhs),
        InfixOperator::RightShift => lhs.right_shift(rhs),
        InfixOperator::Substraction => lhs.substraction(rhs),
        InfixOperator::Sum => lhs.sum(rhs),
    };

    match res {
        Err(()) => Err(Error(EvalError::NonExistentOperation.into(), infix_pos)),
        Ok(obj) => Ok(obj),
    }
}

fn prefix(op: PrefixOperator, obj: Object, prefix_pos: Position) -> Result<Object, Error> {
    let res = match op {
        PrefixOperator::BitwiseNot => obj.bitwise_not(),
        PrefixOperator::LogicNot => obj.logic_not(),
        PrefixOperator::Minus => obj.inverse(),
    };

    match res {
        Err(()) => Err(Error(EvalError::NonExistentOperation.into(), prefix_pos)),
        Ok(obj) => Ok(obj),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;
    use std::vec;

    use super::*;
    use crate::ast::{
        _boolean, _call, _comprehension_list, _comprehension_set, _dummy_pos, _extension_list,
        _extension_set, _for, _function, _infix, _integer, _let_, _pos, _prefix, _prepend,
        _signature, _symbol, _tuple,
    };
    use crate::object::*;

    #[test]
    fn symbol() {
        let node = _symbol("a", _dummy_pos());
        assert_eq!(
            exec(&node, &mut Default::default()),
            Ok(Object::Symbol(Symbol::from("a")))
        );
    }

    #[test]
    fn set_by_extension() {
        let node = _extension_set(
            vec![_symbol("a", _dummy_pos()), _symbol("a", _dummy_pos())],
            _dummy_pos(),
        );

        assert_eq!(
            exec(&node, &mut Default::default()),
            Ok(Object::ExtensionSet(ExtensionSet::from(vec![
                Object::Symbol(Symbol::from("a")),
                Object::Symbol(Symbol::from("a")),
            ]))),
        );
    }

    #[test]
    fn integer_sum() {
        let node = _infix(
            InfixOperator::Sum,
            _integer("0", _dummy_pos()),
            _integer("1", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(&node, &mut Default::default()),
            Ok(Object::Integer(Integer::from("1")))
        );
    }

    #[test]
    fn integer_substraction() {
        let node = &_infix(
            InfixOperator::Substraction,
            _integer("0", _dummy_pos()),
            _integer("1", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Integer(Integer::from(-1)))
        );
    }

    #[test]
    fn integer_product() {
        let node = &_infix(
            InfixOperator::Product,
            _integer("0", _dummy_pos()),
            _integer("1", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Integer(Integer::from("0")))
        );
    }

    #[test]
    fn symbol_comparison() {
        let node = &_infix(
            InfixOperator::Equality,
            _symbol("a", _dummy_pos()),
            _symbol("b", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Boolean(Bool::from(false)))
        );
    }

    #[test]
    fn let_expression() {
        let node = &_let_(
            _signature(_symbol("x", _dummy_pos()), None, _dummy_pos()),
            vec![],
            _integer("0", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Integer(Integer::from(0)))
        );
    }

    #[test]
    fn logic_operators() {
        let node = &_infix(
            InfixOperator::LogicAnd,
            _infix(
                InfixOperator::Or,
                _boolean(true, _dummy_pos()),
                _boolean(false, _dummy_pos()),
                _dummy_pos(),
            ),
            _boolean(false, _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Boolean(Bool::from(false)))
        );
    }

    #[test]
    fn less_leq() {
        let node = &_infix(
            InfixOperator::LogicAnd,
            _infix(
                InfixOperator::Less,
                _integer("0", _dummy_pos()),
                _integer("1", _dummy_pos()),
                _dummy_pos(),
            ),
            _infix(
                InfixOperator::LessEqual,
                _integer("1", _dummy_pos()),
                _integer("1", _dummy_pos()),
                _dummy_pos(),
            ),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Boolean(Bool::from(true)))
        );
    }

    #[test]
    fn greater_geq() {
        let node = &_infix(
            InfixOperator::LogicAnd,
            _infix(
                InfixOperator::Greater,
                _integer("1", _dummy_pos()),
                _integer("0", _dummy_pos()),
                _dummy_pos(),
            ),
            _infix(
                InfixOperator::GreaterEqual,
                _integer("0", _dummy_pos()),
                _integer("0", _dummy_pos()),
                _dummy_pos(),
            ),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Boolean(Bool::from(true)))
        );
    }

    #[test]
    fn neq() {
        let node = &_infix(
            InfixOperator::LogicAnd,
            _infix(
                InfixOperator::NotEquality,
                _integer("1", _dummy_pos()),
                _integer("2", _dummy_pos()),
                _dummy_pos(),
            ),
            _infix(
                InfixOperator::NotEquality,
                _integer("1", _dummy_pos()),
                _boolean(true, _dummy_pos()),
                _dummy_pos(),
            ),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Boolean(Bool::from(true)))
        );
    }

    #[test]
    fn bitwise_and_xor_or() {
        let node = &_infix(
            InfixOperator::Or,
            _infix(
                InfixOperator::BitwiseXor,
                _infix(
                    InfixOperator::BitwiseAnd,
                    _integer("7", _dummy_pos()),
                    _integer("6", _dummy_pos()),
                    _dummy_pos(),
                ),
                _integer("1", _dummy_pos()),
                _dummy_pos(),
            ),
            _integer("0", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Integer(Integer::from(7)))
        );
    }

    #[test]
    fn shifts() {
        let node = &_infix(
            InfixOperator::LeftShift,
            _infix(
                InfixOperator::RightShift,
                _integer("256", _dummy_pos()),
                _integer("4", _dummy_pos()),
                _dummy_pos(),
            ),
            _integer("1", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Integer(Integer::from(32)))
        );
    }

    #[test]
    fn power_and_division() {
        let node = &_infix(
            InfixOperator::Division,
            _infix(
                InfixOperator::Exponentiation,
                _integer("3", _dummy_pos()),
                _integer("2", _dummy_pos()),
                _dummy_pos(),
            ),
            _integer("2", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Integer(Integer::from(4)))
        );
    }

    #[test]
    fn remainder() {
        let node = &_infix(
            InfixOperator::Mod,
            _integer("3", _dummy_pos()),
            _integer("2", _dummy_pos()),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Integer(Integer::from(1)))
        );
    }

    #[test]
    fn prefix() {
        let node = &_prefix(
            PrefixOperator::LogicNot,
            _infix(
                InfixOperator::NotEquality,
                _prefix(
                    PrefixOperator::BitwiseNot,
                    _integer("1", _dummy_pos()),
                    _dummy_pos(),
                ),
                _prefix(
                    PrefixOperator::Minus,
                    _integer("1", _dummy_pos()),
                    _dummy_pos(),
                ),
                _dummy_pos(),
            ),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Default::default()),
            Ok(Object::Boolean(Bool::from(false)))
        );
    }

    #[test]
    fn if_expr() {
        let mut env = Environment::default();
        env.set("a", Object::Integer(Integer::from(-5)));

        let node = &ASTNode::new(
            ASTNodeType_::If(
                Box::new(_infix(
                    InfixOperator::Less,
                    _symbol("a", _dummy_pos()),
                    _integer("0", _dummy_pos()),
                    _dummy_pos(),
                )),
                Box::new(_prefix(
                    PrefixOperator::Minus,
                    _symbol("a", _dummy_pos()),
                    _dummy_pos(),
                )),
                Box::new(_symbol("a", _dummy_pos())),
            ),
            _dummy_pos(),
        );

        assert_eq!(exec(node, &mut env), Ok(Object::Integer(Integer::from(5))));
    }

    #[test]
    fn scope_hierarchy() {
        let mut env = Environment::default();
        env.set("x", Object::Boolean(Bool::from(true)));
        env.push_scope();

        let node = &_symbol("x", _dummy_pos());

        assert_eq!(exec(node, &mut env), Ok(Object::Boolean(Bool::from(true))));
    }

    #[test]
    fn save_value() {
        let mut env = Environment::default();

        let node = &_let_(
            _symbol("x", _dummy_pos()),
            vec![],
            _integer("0", _dummy_pos()),
            _dummy_pos(),
        );

        assert!(exec(node, &mut env).is_ok());

        assert_eq!(env.get("x"), Some(&mut Object::Integer(Integer::from(0))));
    }

    #[test]
    fn tuple() {
        let node = &_tuple(
            vec![_integer("1", _dummy_pos()), _integer("2", _dummy_pos())],
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::Tuple(Tuple::from(vec![
                Object::Integer(Integer::from(1)),
                Object::Integer(Integer::from(2)),
            ]))),
        );
    }

    #[test]
    fn function() {
        let node = &_function(
            vec!["x"],
            vec![_infix(
                InfixOperator::Product,
                _integer("2", _dummy_pos()),
                _symbol("x", _dummy_pos()),
                _dummy_pos(),
            )],
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::Function(object::Function::DefinedFunction(
                DefinedFunction::new(
                    vec![String::from("x"),],
                    vec![_infix(
                        InfixOperator::Product,
                        _integer("2", _dummy_pos()),
                        _symbol("x", _dummy_pos()),
                        _dummy_pos()
                    )],
                )
            ))),
        );
    }

    #[test]
    fn call() {
        let node = &_call(
            _function(
                vec!["x"],
                vec![_infix(
                    InfixOperator::Product,
                    _integer("2", _dummy_pos()),
                    _symbol("x", _dummy_pos()),
                    _dummy_pos(),
                )],
                _dummy_pos(),
            ),
            vec![_integer("1", _dummy_pos())],
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::Integer(Integer::from(2)))
        );
    }

    #[test]
    fn several_params_call() {
        let node = &_call(
            _function(
                vec!["x", "y"],
                vec![_infix(
                    InfixOperator::Sum,
                    _symbol("x", _dummy_pos()),
                    _symbol("y", _dummy_pos()),
                    _dummy_pos(),
                )],
                _dummy_pos(),
            ),
            vec![_integer("1", _dummy_pos()), _integer("2", _dummy_pos())],
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::Integer(Integer::from(3))),
        );
    }

    #[test]
    fn missing_args() {
        let node = &_call(
            _function(
                vec!["x", "y"],
                vec![_infix(
                    InfixOperator::Sum,
                    _symbol("x", _dummy_pos()),
                    _symbol("y", _dummy_pos()),
                    _dummy_pos(),
                )],
                _dummy_pos(),
            ),
            vec![_integer("1", _dummy_pos())],
            _pos(0, 5),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Err(Error(
                EvalError::MissingFunctionArguments.into(),
                _pos(0, 5),
            ))
        );
    }

    #[test]
    fn for_loop() {
        // I do this to test the change of state without assignments
        static ARGS: Mutex<Vec<String>> = Mutex::new(vec![]);

        fn test(args: &[Object]) -> Object {
            ARGS.lock().unwrap().push(args[0].to_string());
            Object::empty_tuple()
        }

        let mut env = Environment::default();
        env.set(
            "f",
            Object::Function(Function::Effect(Effect::new(test, 1))),
        );

        let node = &_for(
            "val",
            _extension_set(
                vec![
                    _integer("1", _dummy_pos()),
                    _integer("2", _dummy_pos()),
                    _integer("3", _dummy_pos()),
                ],
                _dummy_pos(),
            ),
            vec![_call(
                _symbol("f", _dummy_pos()),
                vec![_symbol("val", _dummy_pos())],
                _dummy_pos(),
            )],
            _dummy_pos(),
        );

        assert_eq!(exec(node, &mut env), Ok(Object::empty_tuple()),);

        assert_eq!(
            *ARGS.lock().unwrap(),
            vec![String::from("1"), String::from("2"), String::from("3"),]
        );
    }

    #[test]
    fn comprehension_set() {
        let node = &&_comprehension_set(
            _symbol("k", _dummy_pos()),
            _infix(
                InfixOperator::Greater,
                _symbol("k", _dummy_pos()),
                _integer("1", _dummy_pos()),
                _dummy_pos(),
            ),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::ComprehensionSet(ComprehensionSet::from((
                _symbol("k", _dummy_pos()),
                _infix(
                    InfixOperator::Greater,
                    _symbol("k", _dummy_pos()),
                    _integer("1", _dummy_pos()),
                    _dummy_pos()
                )
            )))),
        );
    }

    #[test]
    fn comprehension_set_question() {
        let node = &_infix(
            InfixOperator::In,
            _integer("1", _dummy_pos()),
            _comprehension_set(
                _symbol("k", _dummy_pos()),
                _infix(
                    InfixOperator::GreaterEqual,
                    _symbol("k", _dummy_pos()),
                    _integer("1", _dummy_pos()),
                    _dummy_pos(),
                ),
                _dummy_pos(),
            ),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::Boolean(crate::object::Bool::from(true))),
        );
    }

    #[test]
    fn extension_list() {
        let node = &_extension_list(vec![_integer("1", _dummy_pos())], _dummy_pos());

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::ExtensionList(crate::object::ExtensionList::from(
                vec![Object::Integer(crate::object::Integer::from(1)),]
            ))),
        )
    }

    #[test]
    fn function_with_code_block() {
        let node = &_call(
            _function(
                vec!["x"],
                vec![
                    _let_(
                        _symbol("y", _dummy_pos()),
                        vec![],
                        _infix(
                            InfixOperator::Product,
                            _integer("2", _dummy_pos()),
                            _symbol("x", _dummy_pos()),
                            _dummy_pos(),
                        ),
                        _dummy_pos(),
                    ),
                    _infix(
                        InfixOperator::Sum,
                        _symbol("y", _dummy_pos()),
                        _integer("1", _dummy_pos()),
                        _dummy_pos(),
                    ),
                ],
                _dummy_pos(),
            ),
            vec![_integer("2", _dummy_pos())],
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::Integer(Integer::from(5))),
        );
    }

    #[test]
    fn comprehension_list() {
        let node = &_comprehension_list(
            _infix(
                InfixOperator::Sum,
                _symbol("k", _dummy_pos()),
                _integer("1", _dummy_pos()),
                _dummy_pos(),
            ),
            _infix(
                InfixOperator::In,
                _symbol("k", _dummy_pos()),
                _extension_list(
                    vec![_integer("0", _dummy_pos()), _integer("1", _dummy_pos())],
                    _dummy_pos(),
                ),
                _dummy_pos(),
            ),
            _dummy_pos(),
        );

        assert_eq!(
            exec(node, &mut Environment::default()),
            Ok(Object::ExtensionList(ExtensionList::from(vec![
                Object::Integer(Integer::from(1)),
                Object::Integer(Integer::from(2)),
            ])))
        );
    }

    #[test]
    fn prepend() {
        let node = _prepend(
            _integer("1", _dummy_pos()),
            _extension_list(vec![_symbol("s", _dummy_pos())], _dummy_pos()),
            _dummy_pos(),
        );

        let obj = Object::ExtensionList(
            vec![
                Object::Integer(Integer::from(1)),
                Object::Symbol(Symbol::from("s")),
            ]
            .into(),
        );

        assert_eq!(exec(&node, &mut Environment::default()), Ok(obj));
    }
}
