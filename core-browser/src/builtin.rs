use std::sync::Mutex;

use komodo::{
    builtin::komodo_assert,
    env::Environment,
    object::{ExternFunction, Function, Object},
};

pub static STDOUT: Mutex<String> = Mutex::new(String::new());
pub static STDIN: Mutex<Vec<String>> = Mutex::new(vec![]);

pub fn standard_env() -> Environment {
    fn komodo_println(args: &[Object]) -> Object {
        let mut guard = STDOUT.lock().unwrap();
        guard.push_str(&args[0].to_string());
        guard.push('\n');

        Object::empty_tuple()
    }

    fn komodo_getln(_args: &[Object]) -> Object {
        let mut guard = STDIN.lock().unwrap();

        let res = guard.pop().unwrap_or_default();

        Object::String(res.into())
    }

    let mut env = Environment::default();
    env.set_inmutable(
        "println",
        Object::Function(Function::Extern(ExternFunction::new(komodo_println, 1))),
    );
    env.set_inmutable(
        "getln",
        Object::Function(Function::Extern(ExternFunction::new(komodo_getln, 0))),
    );

    env.set_inmutable(
        "assert",
        Object::Function(Function::Extern(ExternFunction::new(komodo_assert, 1))),
    );

    env
}
