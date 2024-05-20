/// A file in GodotCfg format.
pub type File<'a> = Vec<Line<'a>>;

type Map<'a, K = Value<'a>> = Vec<(K, Value<'a>)>;

#[derive(Debug)]
pub enum Line<'a> {
    /// A ``// comment`` line.
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
    Array(Vec<Value<'a>>),
    /// A map expression.
    Map(Map<'a>),
    /// An object expression. ``.0`` is object's identifier,
    /// ``.1`` is object's properties.
    Object(&'a str, Map<'a, &'a str>),
    /// A ``PackedByteArray``.
    PackedByteArray(Vec<u8>),
    /// A ``PackedStringArray``.
    PackedStringArray(Vec<&'a str>),
}
