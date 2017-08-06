use super::SSAVariable;
use ::ir::hir::{ Pattern, LambdaEnvIdx };
use ::ir::FunctionIdent;
use ::Atom;

pub mod from_hir;
pub mod to_dot;
pub mod pass;

pub mod cfg;
pub use self::cfg::FunctionCfg;

#[derive(Debug, Clone)]
pub struct BasicBlock {
    label: Label,

    // Meta
    back_refs: Vec<Label>,

    // Main data
    phi_nodes: Vec<Phi>,
    ops: Vec<Op>,
    jumps: Vec<Label>,
}

#[derive(Debug, Clone, Copy)]
pub struct Label(u32);

#[derive(Debug, Clone)]
pub enum Source {
    Variable(SSAVariable),
    Constant(::parser::AtomicLiteral),
}

#[derive(Debug, Clone)]
pub struct Phi {
    entries: Vec<(Label, SSAVariable)>,
    ssa: SSAVariable,
}

#[derive(Debug, Clone)]
pub struct Op {
    kind: OpKind,
    reads: Vec<Source>,
    writes: Vec<SSAVariable>,
}

#[derive(Debug, Clone)]
pub enum OpKind {
    /// Move r[0] into w[0]
    Move,

    /// Calls r[0]:r[1] with args r[2..]
    Call,
    /// Calls r[0] with args r[1..]
    Apply,
    CaptureNamedFunction(::ir::FunctionIdent),

    MakeTuple,
    MakeList,

    MakeClosureEnv {
        env_idx: LambdaEnvIdx,
    },
    BindClosure {
        ident: FunctionIdent,
    },

    Case {
        vars: Vec<SSAVariable>,
        clauses: Vec<Clause>
    },

    Jump,

    PrimOp(Atom),

    ReturnOk,
    ReturnThrow,
}

#[derive(Debug, Clone)]
pub struct Clause {
    patterns: Vec<Pattern>,
}

impl OpKind {

    fn num_jumps(&self) -> Option<usize> {
        match *self {
            OpKind::Call => Some(2),
            OpKind::Apply => Some(2),
            OpKind::Jump => Some(1),
            OpKind::Case { ref clauses, .. } => Some(clauses.len()),
            OpKind::ReturnOk => Some(0),
            OpKind::ReturnThrow => Some(0),
            _ => None,
        }
    }

}