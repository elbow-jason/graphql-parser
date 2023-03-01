use std::fmt;

use crate::format::{format_directives, Displayable, Formatter, Style};

use crate::query::ast::*;

impl<'a> Document<'a>
// where
// T: Text<'a>,
{
    /// Format a document according to style
    pub fn format(&self, style: &Style) -> String {
        let mut formatter = Formatter::new(style);
        self.display(&mut formatter);
        formatter.into_string()
    }
}

fn to_string<T: Displayable>(v: &T) -> String {
    let style = Style::default();
    let mut formatter = Formatter::new(&style);
    v.display(&mut formatter);
    formatter.into_string()
}

impl<'a> Displayable for Document<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        for item in &self.definitions {
            item.display(f);
        }
    }
}

impl<'a> Displayable for Definition<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        match *self {
            Definition::Operation(ref op) => op.display(f),
            Definition::Fragment(ref frag) => frag.display(f),
        }
    }
}

impl<'a> Displayable for OperationDefinition<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        match *self {
            OperationDefinition::SelectionSet(ref set) => set.display(f),
            OperationDefinition::Query(ref q) => q.display(f),
            OperationDefinition::Mutation(ref m) => m.display(f),
            OperationDefinition::Subscription(ref s) => s.display(f),
        }
    }
}

impl<'a> Displayable for FragmentDefinition<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.margin();
        f.indent();
        f.write("fragment ");
        f.write(self.name.as_ref());
        f.write(" ");
        self.type_condition.display(f);
        format_directives(&self.directives, f);
        f.write(" ");
        f.start_block();
        for item in &self.selection_set.items {
            item.display(f);
        }
        f.end_block();
    }
}

impl<'a> Displayable for SelectionSet<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.margin();
        f.indent();
        f.start_block();
        for item in &self.items {
            item.display(f);
        }
        f.end_block();
    }
}

impl<'a> Displayable for Selection<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        match *self {
            Selection::Field(ref fld) => fld.display(f),
            Selection::InlineFragment(ref frag) => frag.display(f),
            Selection::FragmentSpread(ref frag) => frag.display(f),
        }
    }
}

fn format_arguments<'a>(arguments: &[(&'a str, Value<'a>)], f: &mut Formatter)
// where
// T: Text<'a>,
{
    if !arguments.is_empty() {
        f.start_argument_block('(');
        f.start_argument();
        f.write(&arguments[0].0.as_ref());
        f.write(": ");
        arguments[0].1.display(f);
        for arg in &arguments[1..] {
            f.deliniate_argument();
            f.start_argument();
            f.write(&arg.0.as_ref());
            f.write(": ");
            arg.1.display(f);
        }
        f.end_argument_block(')');
    }
}

impl<'a> Displayable for Field<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.indent();
        if let Some(ref alias) = self.alias {
            f.write(alias.as_ref());
            f.write(": ");
        }
        f.write(self.name.as_ref());
        format_arguments(&self.arguments, f);
        format_directives(&self.directives, f);
        if !self.selection_set.items.is_empty() {
            f.write(" ");
            f.start_block();
            for item in &self.selection_set.items {
                item.display(f);
            }
            f.end_block();
        } else {
            f.endline();
        }
    }
}

impl<'a> Displayable for Query<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.margin();
        f.indent();
        f.write("query");
        if let Some(ref name) = self.name {
            f.write(" ");
            f.write(name.as_ref());
        }
        if !self.variable_definitions.is_empty() {
            f.write("(");
            self.variable_definitions[0].display(f);
            for var in &self.variable_definitions[1..] {
                f.write(", ");
                var.display(f);
            }
            f.write(")");
        }
        format_directives(&self.directives, f);
        f.write(" ");
        f.start_block();
        for item in &self.selection_set.items {
            item.display(f);
        }
        f.end_block();
    }
}

impl<'a> Displayable for Mutation<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.margin();
        f.indent();
        f.write("mutation");
        if let Some(ref name) = self.name {
            f.write(" ");
            f.write(name.as_ref());
        }
        if !self.variable_definitions.is_empty() {
            f.write("(");
            self.variable_definitions[0].display(f);
            for var in &self.variable_definitions[1..] {
                f.write(", ");
                var.display(f);
            }
            f.write(")");
        }
        format_directives(&self.directives, f);
        f.write(" ");
        f.start_block();
        for item in &self.selection_set.items {
            item.display(f);
        }
        f.end_block();
    }
}

impl<'a> Displayable for Subscription<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.margin();
        f.indent();
        f.write("subscription");
        if let Some(ref name) = self.name {
            f.write(" ");
            f.write(name.as_ref());
            if !self.variable_definitions.is_empty() {
                f.write("(");
                for var in &self.variable_definitions {
                    var.display(f);
                }
                f.write(")");
            }
        }
        format_directives(&self.directives, f);
        f.write(" ");
        f.start_block();
        for item in &self.selection_set.items {
            item.display(f);
        }
        f.end_block();
    }
}

impl<'a> Displayable for VariableDefinition<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.write("$");
        f.write(self.name.as_ref());
        f.write(": ");
        self.var_type.display(f);
        if let Some(ref default) = self.default_value {
            f.write(" = ");
            default.display(f);
        }
    }
}

impl<'a> Displayable for Type<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        match *self {
            Type::NamedType(ref name) => f.write(name.as_ref()),
            Type::ListType(ref typ) => {
                f.write("[");
                typ.display(f);
                f.write("]");
            }
            Type::NonNullType(ref typ) => {
                typ.display(f);
                f.write("!");
            }
        }
    }
}

impl<'a> Displayable for Value<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        match *self {
            Value::Variable(ref name) => {
                f.write("$");
                f.write(name.as_ref());
            }
            Value::Int(ref num) => f.write(&format!("{}", num.0)),
            Value::Float(val) => f.write(&format!("{}", val)),
            Value::String(ref val) => f.write_quoted(val),
            Value::Boolean(true) => f.write("true"),
            Value::Boolean(false) => f.write("false"),
            Value::Null => f.write("null"),
            Value::Enum(ref name) => f.write(name.as_ref()),
            Value::List(ref items) => {
                f.start_argument_block('[');
                if !items.is_empty() {
                    f.start_argument();
                    items[0].display(f);
                    for item in &items[1..] {
                        f.deliniate_argument();
                        f.start_argument();
                        item.display(f);
                    }
                }
                f.end_argument_block(']');
            }
            Value::Object(ref items) => {
                f.start_argument_block('{');
                let mut first = true;
                for (name, value) in items.iter() {
                    if first {
                        first = false;
                    } else {
                        f.deliniate_argument();
                    }
                    f.start_argument();
                    f.write(name.as_ref());
                    f.write(": ");
                    value.display(f);
                }
                f.end_argument_block('}');
            }
        }
    }
}

impl<'a> Displayable for InlineFragment<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.indent();
        f.write("...");
        if let Some(ref cond) = self.type_condition {
            f.write(" ");
            cond.display(f);
        }
        format_directives(&self.directives, f);
        f.write(" ");
        f.start_block();
        for item in &self.selection_set.items {
            item.display(f);
        }
        f.end_block();
    }
}

impl<'a> Displayable for TypeCondition<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        match *self {
            TypeCondition::On(ref name) => {
                f.write("on ");
                f.write(name.as_ref());
            }
        }
    }
}

impl<'a> Displayable for FragmentSpread<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.indent();
        f.write("...");
        f.write(self.fragment_name.as_ref());
        format_directives(&self.directives, f);
        f.endline();
    }
}

impl<'a> Displayable for Directive<'a>
// where
// T: Text<'a>,
{
    fn display(&self, f: &mut Formatter) {
        f.write("@");
        f.write(self.name.as_ref());
        format_arguments(self.arguments.as_slice(), f);
    }
}

impl_display!(
    'a
    Document,
    Definition,
    OperationDefinition,
    FragmentDefinition,
    SelectionSet,
    Field,
    Query,
    Mutation,
    Subscription,
    VariableDefinition,
    Type,
    Value,
    InlineFragment,
    TypeCondition,
    FragmentSpread,
    Directive,
);
