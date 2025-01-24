use nu_engine::command_prelude::*;

#[derive(Clone)]
pub struct AttrExample;

impl Command for AttrExample {
    fn name(&self) -> &str {
        "attr example"
    }

    fn signature(&self) -> Signature {
        Signature::build("attr example")
            .input_output_types(vec![(
                Type::Nothing,
                Type::Record(
                    [
                        ("description".into(), Type::String),
                        ("example".into(), Type::String),
                    ]
                    .into(),
                ),
            )])
            .allow_variants_without_examples(true)
            .required(
                "description",
                SyntaxShape::String,
                "Description of the example.",
            )
            .required("example", SyntaxShape::String, "Example code snippet.")
            .named(
                "result",
                SyntaxShape::Any,
                "Expected output of example.",
                None,
            )
            .category(Category::Core)
    }

    fn description(&self) -> &str {
        "Attribute for adding examples to custom commands."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let description: Spanned<String> = call.req(engine_state, stack, 0)?;
        let code: Spanned<String> = call.req(engine_state, stack, 1)?;
        let result: Option<Value> = call.get_flag(engine_state, stack, "result")?;

        let mut rec = record! {
            "description" => Value::string(description.item, description.span),
            "example" => Value::string(code.item, code.span),
        };
        if let Some(result) = result {
            rec.push("result", result);
        }

        Ok(Value::record(rec, call.head).into_pipeline_data())
    }

    fn run_const(
        &self,
        working_set: &StateWorkingSet,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let description: Spanned<String> = call.req_const(working_set, 0)?;
        let code: Spanned<String> = call.req_const(working_set, 1)?;
        let result: Option<Value> = call.get_flag_const(working_set, "result")?;

        let mut rec = record! {
            "description" => Value::string(description.item, description.span),
            "example" => Value::string(code.item, code.span),
        };
        if let Some(result) = result {
            rec.push("result", result);
        }

        Ok(Value::record(rec, call.head).into_pipeline_data())
    }

    fn is_const(&self) -> bool {
        true
    }
}
