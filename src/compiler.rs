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

    pub fn compile(&mut self, _input: &[u8]) -> Result<*const u8, String> {
        let int_type = self.module.target_config().pointer_type();

        self.ctx.func.signature.params.push(AbiParam::new(int_type));
        self.ctx.func.signature.params.push(AbiParam::new(int_type));
        self.ctx
            .func
            .signature
            .returns
            .push(AbiParam::new(int_type));

        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
        let entry_ebb = builder.create_ebb();
        // pass args to block
        builder.append_ebb_params_for_function_params(entry_ebb);
        // generate code for it
        builder.switch_to_block(entry_ebb);
        // no other generated code will call this so we can seal it
        builder.seal_block(entry_ebb);

        let args = builder.ebb_params(entry_ebb);
        let arg1 = args[0].clone();
        let arg2 = args[1].clone();

        let result = builder.ins().iadd(arg1, arg2);
        builder.ins().return_(&[result]);
        builder.finalize();

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
