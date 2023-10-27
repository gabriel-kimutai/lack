
use crate::parser::NodeExit;

pub struct Generator(String);

impl Generator {
    pub fn generate(root: NodeExit) -> String {
        let mut _output: String = String::new();
        _output = String::new();
        _output.push_str("global _start\n_start:\n");
        _output.push_str("    mov rax, 60\n");
        _output.push_str(format!("    mov rdi, {}\n", root.node_expr.int_lit).as_str());
        _output.push_str("    syscall");
        
        return _output;
    }
}