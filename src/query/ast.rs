//! Query Language Abstract Syntax Tree (AST)
//!
//! The types and fields here resemble official [graphql grammar] whenever it
//! makes sense for rust.
//!
//! [graphql grammar]: http://facebook.github.io/graphql/October2016/#sec-Appendix-Grammar-Summary
//!
pub use crate::common::{Directive, Number, Type, Value};
use crate::position::Pos;

/// Root of query data
#[derive(Debug, Clone, PartialEq)]
pub struct Document<'a> {
    pub definitions: Vec<Definition<'a>>,
}

// impl<'a> Document<'a> {
//     pub fn into_static(self) -> Document<'static, String> {
//         // To support both reference and owned values in the AST,
//         // all string data is represented with the ::common::Str<'a>
//         // wrapper type.
//         // This type must carry the lifetime of the query string,
//         // and is stored in a PhantomData value on the Str type.
//         // When using owned String types, the actual lifetime of
//         // the Ast nodes is 'static, since no references are kept,
//         // but the nodes will still carry the input lifetime.
//         // To continue working with Document<String> in a owned fasion
//         // the lifetime needs to be transmuted to 'static.
//         //
//         // This is safe because no references are present.
//         // Just the PhantomData lifetime reference is transmuted away.
//         unsafe { std::mem::transmute::<_, Document<'static, String>>(self) }
//     }
// }

#[derive(Debug, Clone, PartialEq)]
pub enum Definition<'a> {
    Operation(OperationDefinition<'a>),
    Fragment(FragmentDefinition<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentDefinition<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub type_condition: TypeCondition<'a>,
    pub directives: Vec<Directive<'a>>,
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperationDefinition<'a> {
    SelectionSet(SelectionSet<'a>),
    Query(Query<'a>),
    Mutation(Mutation<'a>),
    Subscription(Subscription<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Query<'a> {
    pub position: Pos,
    pub name: Option<&'a str>,
    pub variable_definitions: Vec<VariableDefinition<'a>>,
    pub directives: Vec<Directive<'a>>,
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mutation<'a> {
    pub position: Pos,
    pub name: Option<&'a str>,
    pub variable_definitions: Vec<VariableDefinition<'a>>,
    pub directives: Vec<Directive<'a>>,
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Subscription<'a> {
    pub position: Pos,
    pub name: Option<&'a str>,
    pub variable_definitions: Vec<VariableDefinition<'a>>,
    pub directives: Vec<Directive<'a>>,
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionSet<'a> {
    pub span: (Pos, Pos),
    pub items: Vec<Selection<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDefinition<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub var_type: Type<'a>,
    pub default_value: Option<Value<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Selection<'a> {
    Field(Field<'a>),
    FragmentSpread(FragmentSpread<'a>),
    InlineFragment(InlineFragment<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a> {
    pub position: Pos,
    pub alias: Option<&'a str>,
    pub name: &'a str,
    pub arguments: Vec<(&'a str, Value<'a>)>,
    pub directives: Vec<Directive<'a>>,
    pub selection_set: SelectionSet<'a>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentSpread<'a> {
    pub position: Pos,
    pub fragment_name: &'a str,
    pub directives: Vec<Directive<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeCondition<'a> {
    On(&'a str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineFragment<'a> {
    pub position: Pos,
    pub type_condition: Option<TypeCondition<'a>>,
    pub directives: Vec<Directive<'a>>,
    pub selection_set: SelectionSet<'a>,
}
