use std::str::FromStr;

use thiserror::Error;

pub use crate::common::{Directive, Type, Value};
use crate::position::Pos;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Document<'a>
// where
// T: Text<'a>,
{
    pub definitions: Vec<Definition<'a>>,
}

// impl<'a> Document<'a, String> {
//     pub fn into_static(self) -> Document<'static, String> {
//         // To support both reference and owned values in the AST,
//         // all string data is represented with the ::common::Str<'a>
//         // wrapper type.
//         // This type must carry the liftetime of the schema string,
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
    SchemaDefinition(SchemaDefinition<'a>),
    TypeDefinition(TypeDefinition<'a>),
    TypeExtension(TypeExtension<'a>),
    DirectiveDefinition(DirectiveDefinition<'a>),
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SchemaDefinition<'a> {
    pub position: Pos,
    pub directives: Vec<Directive<'a>>,
    pub query: Option<&'a str>,
    pub mutation: Option<&'a str>,
    pub subscription: Option<&'a str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDefinition<'a> {
    Scalar(ScalarType<'a>),
    Object(ObjectType<'a>),
    Interface(InterfaceType<'a>),
    Union(UnionType<'a>),
    Enum(EnumType<'a>),
    InputObject(InputObjectType<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExtension<'a> {
    Scalar(ScalarTypeExtension<'a>),
    Object(ObjectTypeExtension<'a>),
    Interface(InterfaceTypeExtension<'a>),
    Union(UnionTypeExtension<'a>),
    Enum(EnumTypeExtension<'a>),
    InputObject(InputObjectTypeExtension<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarType<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
}

impl<'a> ScalarType<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScalarTypeExtension<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
}

impl<'a> ScalarTypeExtension<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectType<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub implements_interfaces: Vec<&'a str>,
    pub directives: Vec<Directive<'a>>,
    pub fields: Vec<Field<'a>>,
}

impl<'a> ObjectType<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectTypeExtension<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub implements_interfaces: Vec<&'a str>,
    pub directives: Vec<Directive<'a>>,
    pub fields: Vec<Field<'a>>,
}

impl<'a> ObjectTypeExtension<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub arguments: Vec<InputValue<'a>>,
    pub field_type: Type<'a>,
    pub directives: Vec<Directive<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputValue<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub value_type: Type<'a>,
    pub default_value: Option<Value<'a>>,
    pub directives: Vec<Directive<'a>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceType<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub implements_interfaces: Vec<&'a str>,
    pub directives: Vec<Directive<'a>>,
    pub fields: Vec<Field<'a>>,
}

impl<'a> InterfaceType<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceTypeExtension<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub implements_interfaces: Vec<&'a str>,
    pub directives: Vec<Directive<'a>>,
    pub fields: Vec<Field<'a>>,
}

impl<'a> InterfaceTypeExtension<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            name,
            implements_interfaces: vec![],
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionType<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
    pub types: Vec<&'a str>,
}

impl<'a> UnionType<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
            types: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionTypeExtension<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
    pub types: Vec<&'a str>,
}

impl<'a> UnionTypeExtension<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
            types: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
    pub values: Vec<EnumValue<'a>>,
}

impl<'a> EnumType<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
            values: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumValue<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
}

impl<'a> EnumValue<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumTypeExtension<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
    pub values: Vec<EnumValue<'a>>,
}

impl<'a> EnumTypeExtension<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
            values: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectType<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
    pub fields: Vec<InputValue<'a>>,
}

impl<'a> InputObjectType<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputObjectTypeExtension<'a> {
    pub position: Pos,
    pub name: &'a str,
    pub directives: Vec<Directive<'a>>,
    pub fields: Vec<InputValue<'a>>,
}

impl<'a> InputObjectTypeExtension<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            name,
            directives: vec![],
            fields: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DirectiveLocation {
    // executable
    Query,
    Mutation,
    Subscription,
    Field,
    FragmentDefinition,
    FragmentSpread,
    InlineFragment,

    // type_system
    Schema,
    Scalar,
    Object,
    FieldDefinition,
    ArgumentDefinition,
    Interface,
    Union,
    Enum,
    EnumValue,
    InputObject,
    InputFieldDefinition,
    VariableDefinition,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DirectiveDefinition<'a> {
    pub position: Pos,
    pub description: Option<String>,
    pub name: &'a str,
    pub arguments: Vec<InputValue<'a>>,
    pub repeatable: bool,
    pub locations: Vec<DirectiveLocation>,
}

impl<'a> DirectiveDefinition<'a>
// where
// T: Text<'a>,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            position: Pos::default(),
            description: None,
            name,
            arguments: vec![],
            repeatable: false,
            locations: vec![],
        }
    }
}

impl DirectiveLocation {
    /// Returns GraphQL syntax compatible name of the directive
    pub fn as_str(&self) -> &'static str {
        use self::DirectiveLocation::*;
        match *self {
            Query => "QUERY",
            Mutation => "MUTATION",
            Subscription => "SUBSCRIPTION",
            Field => "FIELD",
            FragmentDefinition => "FRAGMENT_DEFINITION",
            FragmentSpread => "FRAGMENT_SPREAD",
            InlineFragment => "INLINE_FRAGMENT",
            Schema => "SCHEMA",
            Scalar => "SCALAR",
            Object => "OBJECT",
            FieldDefinition => "FIELD_DEFINITION",
            ArgumentDefinition => "ARGUMENT_DEFINITION",
            Interface => "INTERFACE",
            Union => "UNION",
            Enum => "ENUM",
            EnumValue => "ENUM_VALUE",
            InputObject => "INPUT_OBJECT",
            InputFieldDefinition => "INPUT_FIELD_DEFINITION",
            VariableDefinition => "VARIABLE_DEFINITION",
        }
    }

    /// Returns `true` if this location is for queries (execution)
    pub fn is_query(&self) -> bool {
        use self::DirectiveLocation::*;
        match *self {
            Query | Mutation | Subscription | Field | FragmentDefinition | FragmentSpread
            | InlineFragment => true,

            Schema | Scalar | Object | FieldDefinition | ArgumentDefinition | Interface | Union
            | Enum | EnumValue | InputObject | InputFieldDefinition | VariableDefinition => false,
        }
    }

    /// Returns `true` if this location is for schema
    pub fn is_schema(&self) -> bool {
        !self.is_query()
    }
}

#[derive(Debug, Error)]
#[error("invalid directive location")]
pub struct InvalidDirectiveLocation;

impl FromStr for DirectiveLocation {
    type Err = InvalidDirectiveLocation;
    fn from_str(s: &str) -> Result<DirectiveLocation, InvalidDirectiveLocation> {
        use self::DirectiveLocation::*;
        let val = match s {
            "QUERY" => Query,
            "MUTATION" => Mutation,
            "SUBSCRIPTION" => Subscription,
            "FIELD" => Field,
            "FRAGMENT_DEFINITION" => FragmentDefinition,
            "FRAGMENT_SPREAD" => FragmentSpread,
            "INLINE_FRAGMENT" => InlineFragment,
            "SCHEMA" => Schema,
            "SCALAR" => Scalar,
            "OBJECT" => Object,
            "FIELD_DEFINITION" => FieldDefinition,
            "ARGUMENT_DEFINITION" => ArgumentDefinition,
            "INTERFACE" => Interface,
            "UNION" => Union,
            "ENUM" => Enum,
            "ENUM_VALUE" => EnumValue,
            "INPUT_OBJECT" => InputObject,
            "INPUT_FIELD_DEFINITION" => InputFieldDefinition,
            "VARIABLE_DEFINITION" => VariableDefinition,
            _ => return Err(InvalidDirectiveLocation),
        };

        Ok(val)
    }
}
