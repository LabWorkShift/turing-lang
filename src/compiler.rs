use crate::parser::{Node, Parser};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{BasicValue, FloatValue, IntValue};
use std::collections::HashMap;

pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, BasicValue<'ctx>>,
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();

        Self {
            context,
            module,
            builder,
            variables: HashMap::new(),
        }
    }

    pub fn compile(&mut self, node: Node) -> Result<(), String> {
        match node {
            Node::Program(statements) => {
                let i64_type = self.context.i64_type();
                let fn_type = i64_type.fn_type(&[], false);
                let function = self.module.add_function("main", fn_type, None);
                let basic_block = self.context.append_basic_block(function, "entry");
                self.builder.position_at_end(basic_block);

                for statement in statements {
                    self.compile_node(statement)?;
                }

                if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                    self.builder.build_return(Some(&i64_type.const_int(0, false)));
                }

                Ok(())
            }
            _ => Err("Expected program".to_string()),
        }
    }

    fn compile_node(&mut self, node: Node) -> Result<BasicValue<'ctx>, String> {
        match node {
            Node::IntegerLiteral(value) => {
                let int_val = self.context.i64_type().const_int(value as u64, false);
                Ok(int_val.as_basic_value_enum())
            }
            Node::FloatLiteral(value) => {
                let float_val = self.context.f64_type().const_float(value);
                Ok(float_val.as_basic_value_enum())
            }
            Node::BinaryOperation { left, operator, right } => {
                let lhs = self.compile_node(*left)?;
                let rhs = self.compile_node(*right)?;

                match operator.as_str() {
                    "+" => Ok(self.builder.build_int_add(
                        lhs.into_int_value(),
                        rhs.into_int_value(),
                        "addtmp",
                    ).as_basic_value_enum()),
                    "-" => Ok(self.builder.build_int_sub(
                        lhs.into_int_value(),
                        rhs.into_int_value(),
                        "subtmp",
                    ).as_basic_value_enum()),
                    "*" => Ok(self.builder.build_int_mul(
                        lhs.into_int_value(),
                        rhs.into_int_value(),
                        "multmp",
                    ).as_basic_value_enum()),
                    "/" => Ok(self.builder.build_int_signed_div(
                        lhs.into_int_value(),
                        rhs.into_int_value(),
                        "divtmp",
                    ).as_basic_value_enum()),
                    _ => Err(format!("Unknown operator: {}", operator)),
                }
            }
            _ => Err("Unsupported node type".to_string()),
        }
    }

    pub fn get_module(&self) -> &Module<'ctx> {
        &self.module
    }
}
