use apollo_compiler::parser::SourceMap;
use apollo_compiler::Node;

use super::Code;
use super::Message;
use super::Name;
use super::Value;
use crate::sources::connect::spec::schema::CONNECT_HTTP_ARGUMENT_DELETE_METHOD_NAME;
use crate::sources::connect::spec::schema::CONNECT_HTTP_ARGUMENT_GET_METHOD_NAME;
use crate::sources::connect::spec::schema::CONNECT_HTTP_ARGUMENT_PATCH_METHOD_NAME;
use crate::sources::connect::spec::schema::CONNECT_HTTP_ARGUMENT_POST_METHOD_NAME;
use crate::sources::connect::spec::schema::CONNECT_HTTP_ARGUMENT_PUT_METHOD_NAME;
use crate::sources::connect::validation::coordinates::HTTPCoordinate;

pub(super) fn validate_http_method_arg<'arg>(
    http_arg: &'arg [(Name, Node<Value>)],
    coordinate: HTTPCoordinate,
    http_arg_node: &Node<Value>,
    source_map: &SourceMap,
) -> Result<&'arg (Name, Node<Value>), Message> {
    let mut methods = http_arg
        .iter()
        .filter(|(method, _)| {
            [
                CONNECT_HTTP_ARGUMENT_GET_METHOD_NAME,
                CONNECT_HTTP_ARGUMENT_POST_METHOD_NAME,
                CONNECT_HTTP_ARGUMENT_PUT_METHOD_NAME,
                CONNECT_HTTP_ARGUMENT_PATCH_METHOD_NAME,
                CONNECT_HTTP_ARGUMENT_DELETE_METHOD_NAME,
            ]
            .contains(method)
        })
        .peekable();

    let Some(method) = methods.next() else {
        return Err(Message {
            code: Code::MissingHttpMethod,
            message: format!("{coordinate} must specify an HTTP method.",),
            locations: http_arg_node
                .line_column_range(source_map)
                .into_iter()
                .collect(),
        });
    };

    if methods.peek().is_some() {
        let locations = method
            .1
            .line_column_range(source_map)
            .into_iter()
            .chain(methods.filter_map(|(_, node)| node.line_column_range(source_map)))
            .collect();
        return Err(Message {
            code: Code::MultipleHttpMethods,
            message: format!("{coordinate} cannot specify more than one HTTP method.",),
            locations,
        });
    }

    Ok(method)
}
