use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, ReturnSuccess, ReturnValue, Signature, UntaggedValue, Value, Dictionary, SyntaxShape
};

struct PrintArgs;

impl Plugin for PrintArgs {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("print_args")
           .rest(   // accept arbitrary number of arguments
               SyntaxShape::Any,
               "Whatever arguments"
               )
           .desc("Example plugin printing all args").filter())  // don't forget this .filter()
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        // positional args
        let pos_args = call_info.args.positional;
        if let Some(value_list) = pos_args {
            let mut map = indexmap::IndexMap::new();
            let mut num = 0;
            for value in value_list {
                map.insert(format!("arg[{}]", num), value);
                num += 1;
            }
            return Ok(vec![ReturnSuccess::value(Value::new(UntaggedValue::Row(Dictionary::from(map)),
                                                           call_info.name_tag))]);
        }
        Ok(vec![])
    }

    fn filter(&mut self, _: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![])
    }
}

fn main() {
    serve_plugin(&mut PrintArgs{});
}
