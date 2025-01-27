use crate::prelude::*;
use biome_json_syntax::JsonArrayValue;
use biome_json_syntax::JsonArrayValueFields;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArrayValue;
impl FormatNodeRule<JsonArrayValue> for FormatJsonArrayValue {
    fn fmt_fields(&self, node: &JsonArrayValue, f: &mut JsonFormatter) -> FormatResult<()> {
        let JsonArrayValueFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                group(&soft_block_indent(&elements.format())),
                r_brack_token.format()
            ]
        )
    }
}
