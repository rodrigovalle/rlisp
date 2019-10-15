use crate::ast::AstNode;
use crate::env::Env;

macro_rules! nil {
    () => {
        AstNode::SExpr(Vec::new())
    };
}

// helper function for implementing builtins:
// takes a number out of an AstNode
fn unwrap_number(item: &AstNode) -> i64 {
    if let AstNode::Number(i) = item {
        *i
    } else {
        panic!("{:?} is not a number", item)
    }
}

pub struct Eval<'i, 'e> {
    env: Env<&'i str, AstNode<'e>>,
}

impl<'i, 'e> Eval<'i, 'e> {
    pub fn new() -> Eval<'i, 'e> {
        Eval {
            env: Env::empty(),
        }
    }

    pub fn eval(&mut self, node: &'i AstNode<'i>) -> AstNode<'e> {
        match node {
            AstNode::SExpr(node_list) => {
                if let Some((fn_name, args)) = node_list.split_first() {
                    // functional application of a builtin
                    self.eval_builtin(fn_name, args)
                } else {
                    // if the expression is empty then return nil
                    nil!()
                }
            }
            AstNode::Number(number) => AstNode::Number(*number),
            AstNode::Symbol(symbol) => {
                let value = self.env.get(symbol);
                if let Some(v) = value {
                    v.clone()
                } else {
                    panic!("no such variable in scope '{}'", symbol);
                }
            }
        }
    }

    fn evlis(&mut self, args: &'i [AstNode]) -> Vec<AstNode<'e>> {
        args.iter().map(|e| self.eval(e)).collect()
    }

    fn eval_builtin(
        &mut self,
        first_node: &'i AstNode,
        args: &'i [AstNode],
    ) -> AstNode<'e> {
        if let AstNode::Symbol(fn_name) = first_node {
            match *fn_name {
                "+" => builtin_add(&self.evlis(args)),
                "-" => builtin_sub(&self.evlis(args)),
                "def!" => self.builtin_def(args),
                unknown_op => panic!("unrecognized operator {}", unknown_op),
            }
        } else {
            panic!("'{:?}' is not a valid function")
        }
    }

    fn builtin_def(&mut self, raw_args: &'i [AstNode]) -> AstNode<'e> {
        if let [AstNode::Symbol(var), expr] = raw_args {
            let val = self.eval(&expr);
            self.env.put(var, val);
            nil!()
        } else {
            panic!("'def!' received unexpected arguments: {:?}", raw_args)
        }
    }
}

fn builtin_add<'i, 'o>(args: &'i [AstNode]) -> AstNode<'o> {
    let sum = args.iter().map(unwrap_number).sum();
    AstNode::Number(sum)
}

fn builtin_sub<'i, 'o>(args: &'i [AstNode]) -> AstNode<'o> {
    match args.split_at(1) {
        ([AstNode::Number(head)], []) => AstNode::Number(-head),
        ([AstNode::Number(head)], tail) => {
            let result =
                tail.iter().map(unwrap_number).fold(*head, |acc, i| acc - i);
            AstNode::Number(result)
        }
        _ => panic!("'-' Received unexpected arguments {:?}", args),
    }
}

#[cfg(test)]
mod eval_test {
    use super::*;
    use crate::lex::Parse;

    fn eval_expect(input: &'static str, expect: AstNode) {
        let (result, _) = AstNode::parse(input).expect("parse error");
        let mut eval = Eval::new();
        assert_eq!(eval.eval(&result), expect, "input: {}", input);
    }

    #[test]
    fn test_add() {
        let tests = vec![
            ("(+ 1 1)", AstNode::Number(2)),
            ("(+ 2 1)", AstNode::Number(3)),
            ("(+ 0)", AstNode::Number(0)),
            ("(+ -1 0)", AstNode::Number(-1)),
            ("(+ 1 -0)", AstNode::Number(1)),
            ("(+ 1 2 3 4)", AstNode::Number(10)),
        ];
        for (expr, result) in tests {
            eval_expect(expr, result);
        }
    }

    #[test]
    fn test_sub() {
        let tests = vec![
            ("(- 1 1)", AstNode::Number(0)),
            ("(- 2 1)", AstNode::Number(1)),
            ("(- 1 2)", AstNode::Number(-1)),
            ("(- 2)", AstNode::Number(-2)),
            ("(- 1 2 3 4)", AstNode::Number(-8)),
        ];
        for (expr, result) in tests {
            eval_expect(expr, result);
        }
    }

    #[test]
    fn test_def() {
        // PANIC: tries to evaluate 'a' when we call evlis
        // When we see def! we actually don't want to evaluate like normal.
        // Maybe there's a nice way to define an 'Eval' trait so builtins can
        // define/override how they want to be evaluated.
        eval_expect("(def! a (+ 1 2))", nil!())
    }
}
