mod vm;

fn main() {
    let my_code = vec![vm::Bytecode::ICONST as i32, 99,  vm::Bytecode::PRINT as i32, vm::Bytecode::HALT as i32];
    let mut my_vm = vm::new_vm(my_code, 0, 100);
    my_vm.cpu();
}
