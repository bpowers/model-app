extern crate lalrpop;

fn main() {
  lalrpop::process_root().unwrap();
  prost_build::compile_protos(&["src/ast.proto"],
                              &["src"]).unwrap();
}
