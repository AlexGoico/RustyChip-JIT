use crate::runtime;
use cranelift::prelude::*;
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};

pub struct Compiler {
    builder_context: FunctionBuilderContext,
    ctx: codegen::Context,
    _data_ctx: DataContext,
    module: Module<SimpleJITBackend>,
}

impl Compiler {
    pub fn new() -> Self {
        #[cfg(windows)]
        compile_error!("windows calling conventions probably not working");

        let builder = SimpleJITBuilder::new();
        let module = Module::new(builder);
        Self {
            builder_context: FunctionBuilderContext::new(),
            ctx: module.make_context(),
            _data_ctx: DataContext::new(),
            module,
        }
    }

    pub fn compile(
        &mut self,
        runtime_ctx: &runtime::Ctx,
        _input: &[u8],
    ) -> Result<*const u8, String> {
        let int_type = self.module.target_config().pointer_type();

        self.ctx.func.signature.params.push(AbiParam::new(int_type));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_ebb = builder.create_ebb();
        // pass args to block
        builder.append_ebb_params_for_function_params(entry_ebb);
        // generate code for it
        builder.switch_to_block(entry_ebb);
        // no other generated code will call this so we can seal it
        builder.seal_block(entry_ebb);

        {
            let args = builder.ebb_params(entry_ebb);
            let runtime_ctx_ptr = args[0].clone();
            let mut op_trans = OpcodeTranslator {
                int_type,
                builder,
                runtime_ctx: runtime_ctx,
                runtime_ctx_ptr,
                module: &mut self.module,
            };
            op_trans.translate_op(0x00E0);
            op_trans.builder.ins().return_(&[]);
        }

        let id = self
            .module
            .declare_function("entry", Linkage::Export, &self.ctx.func.signature)
            .map_err(|e| e.to_string())?;

        self.module
            .define_function(id, &mut self.ctx)
            .map_err(|e| e.to_string())?;
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions();

        let code = self.module.get_finalized_function(id);
        Ok(code)
    }
}

struct OpcodeTranslator<'a> {
    int_type: types::Type,
    builder: FunctionBuilder<'a>,
    runtime_ctx: &'a runtime::Ctx,
    runtime_ctx_ptr: Value,
    module: &'a mut Module<SimpleJITBackend>,
}

impl<'a> OpcodeTranslator<'a> {
    fn translate_op(&mut self, op: u16) -> Option<Value> {
        match op {
            // CLS
            0x00E0 => {
                let screen_offset = self.runtime_ctx.screen_offset();
                let screen_size = self.runtime_ctx.screen_size();

                let target_conf = cranelift::prelude::isa::TargetFrontendConfig {
                    default_call_conv: cranelift::prelude::isa::CallConv::Fast,
                    pointer_width: target_lexicon::PointerWidth::U64,
                };
                let screen_loc =
                    Value::from_u32(self.runtime_ctx_ptr.as_u32() + screen_offset as u32);
                self.builder
                    .emit_small_memset(target_conf, screen_loc, 0, screen_size as u64, 1);
                None
                /*
                let result = builder.ins().iadd(arg1, arg2);
                        builder.ins().return_(&[result]);
                        builder.finalize();
                */
            }
            _ => unimplemented!(),
        }
    }
}
