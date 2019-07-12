use libeir_ir::{ FunctionBuilder, Value, Block };

pub struct ExceptionHandlerStack {
    stack: Vec<(Value, bool)>,
}
impl ExceptionHandlerStack {

    pub fn new() -> Self {
        ExceptionHandlerStack {
            stack: vec![],
        }
    }

    pub fn push_handler(&mut self, handler: Value) {
        self.stack.push((handler, true));
    }

    pub fn push_anon_handler(&mut self, handler: Value) {
        self.stack.push((handler, false));
    }

    pub fn pop_handler(&mut self) {
        self.stack.pop();
    }

    pub fn make_error_jump(&self, b: &mut FunctionBuilder, block: Block, typ: Value, error: Value, trace: Value) {
        let (handler, has_arg) = self.stack.last().unwrap();
        if *has_arg {
            b.op_call(block, *handler, &[typ, error, trace])
        } else {
            b.op_call(block, *handler, &[])
        }
    }

    pub fn finish(&self) {
        assert!(self.stack.len() == 0);
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

}