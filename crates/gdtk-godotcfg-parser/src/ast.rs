/// A file in GodotCfg format.
pub type File<'a> = Vec<Line<'a>>;

type Array<'a, T = Value<'a>> = Vec<T>;
type Map<'a, K = Value<'a>, V = Value<'a>> = Vec<(K, V)>;

#[derive(Debug)]
pub enum Line<'a> {
    /// A ``// comment``.
    Comment(&'a str),
    /// A ``[section param="value"]``.
    Section(&'a str, Map<'a, &'a str>),
    /// A ``parameter=value``.
    Parameter(&'a str, Value<'a>),
}

#[derive(Debug)]
pub enum Value<'a> {
    /// A ``null``.
    Null,
    /// A ``true`` or a ``false``.
    Boolean(bool),
    /// An integer literal.
    Integer(i32),
    /// A float literal.
    Float(f64),
    /// A string literal.
    String(&'a str),
    /// An array expression.
    Array(Array<'a>),
    /// A map expression.
    Map(Map<'a>),
    /// An object expression. ``.0`` is object's identifier, ``.1`` is object's properties.
    Object(&'a str, Map<'a, &'a str>),
    /// An object instance expression. The difference between this and [Value::Object] is
    /// that `Object` constructs an object directly using a class name and a list of
    /// properties, while `ObjectInstance` more resembles how you would create an object
    /// instance in GDScript, by passing arguments to a class' `_init()`.
    ObjectInstance(&'a str, Array<'a>),
}
