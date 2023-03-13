use cmov::Cmov;

use crate::lexer::Kind;
#[allow(clippy::enum_glob_use)]
use crate::lexer::Kind::*;

mod hash {
    /// Concatenate the first 2 bytes and the last 2 bytes of a slice into a single
    /// 32-bit integer as an efficient hashing function of JS keywords.
    /// This approach is backed by the observation that the byte-sequence formed by taking
    /// the first 2 and last 2 bytes of JS keywords are unique.
    ///
    /// SAFETY:
    /// key.len() >= 2
    #[inline]
    pub unsafe fn extract_first_and_last_two_bytes(key: &[u8]) -> u32 {
        unsafe fn read_u16(input: &[u8]) -> u16 {
            u16::from(*input.get_unchecked(0)) << 8 | u16::from(*input.get_unchecked(1))
        }
        // read first 2 bytes in a u16
        let first = read_u16(key);
        let last_bytes = &key[key.len() - 2..];
        let last = read_u16(last_bytes);
        u32::from(first) | u32::from(last) << 16
    }

    /// Use a multiplicative linear congruential generator (MLCG) to map the hash value
    /// uniformly. Different values of seed are tested to find one with no collisions.
    #[inline]
    pub fn hash_u32(input: u32, seed: u32) -> u32 {
        // The Magic number is taken from
        // https://www.ams.org/journals/mcom/1999-68-225/S0025-5718-99-00996-5/S0025-5718-99-00996-5.pdf
        const MAGIC: u64 = 887_987_685;
        let hash = input ^ seed;
        ((u64::from(hash) * MAGIC) >> 32) as u32
    }
}

const HASH_TABLE_SIZE: usize = 512usize;
const HASH_TABLE_SEED: u32 = 7_484_039u32;
static KIND_TABLE: [Kind; HASH_TABLE_SIZE] = [
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Export,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Enum,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    With,
    Default,
    BigInt,
    Ident,
    Out,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Case,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Readonly,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Infer,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Finally,
    Ident,
    Ident,
    Ident,
    Constructor,
    Ident,
    Ident,
    Ident,
    Interface,
    Ident,
    While,
    Ident,
    Void,
    Const,
    Global,
    Ident,
    Ident,
    Override,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Let,
    Set,
    Ident,
    Ident,
    Never,
    Ident,
    Ident,
    Number,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Intrinsic,
    Ident,
    Ident,
    Ident,
    Ident,
    Do,
    Ident,
    Ident,
    Ident,
    Undefined,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    KeyOf,
    Ident,
    Namespace,
    Get,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Await,
    Ident,
    Ident,
    Ident,
    Ident,
    Typeof,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Yield,
    Ident,
    Private,
    Ident,
    Ident,
    Ident,
    In,
    Ident,
    Ident,
    Ident,
    Throw,
    Break,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Extends,
    Ident,
    Accessor,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    False,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Delete,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Import,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Super,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Symbol,
    Ident,
    Ident,
    Ident,
    Ident,
    String,
    Abstract,
    Ident,
    Async,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Meta,
    Ident,
    Function,
    For,
    Ident,
    Ident,
    Instanceof,
    Ident,
    Ident,
    Object,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Var,
    Require,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Type,
    Ident,
    As,
    Ident,
    Ident,
    Ident,
    Catch,
    Ident,
    Ident,
    New,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Static,
    Ident,
    True,
    Ident,
    Ident,
    Try,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Assert,
    Ident,
    Ident,
    Unknown,
    Debugger,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    If,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    This,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Unique,
    Public,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Any,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Of,
    Ident,
    Continue,
    Ident,
    Ident,
    Else,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Implements,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Target,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Boolean,
    Ident,
    Ident,
    Protected,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Declare,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Null,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Is,
    Ident,
    Package,
    Ident,
    Return,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Module,
    Ident,
    Class,
    Ident,
    From,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Switch,
    Asserts,
    Ident,
    Ident,
    Ident,
    Satisfies,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
    Ident,
];
static STR_TABLE: [&'static str; HASH_TABLE_SIZE] = [
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "export",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "enum",
    "",
    "",
    "",
    "",
    "",
    "",
    "with",
    "default",
    "bigint",
    "",
    "out",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "case",
    "",
    "",
    "",
    "",
    "",
    "",
    "readonly",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "infer",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "finally",
    "",
    "",
    "",
    "constructor",
    "",
    "",
    "",
    "interface",
    "",
    "while",
    "",
    "void",
    "const",
    "global",
    "",
    "",
    "override",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "let",
    "set",
    "",
    "",
    "never",
    "",
    "",
    "number",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "intrinsic",
    "",
    "",
    "",
    "",
    "do",
    "",
    "",
    "",
    "undefined",
    "",
    "",
    "",
    "",
    "",
    "",
    "keyof",
    "",
    "namespace",
    "get",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "await",
    "",
    "",
    "",
    "",
    "typeof",
    "",
    "",
    "",
    "",
    "",
    "yield",
    "",
    "private",
    "",
    "",
    "",
    "in",
    "",
    "",
    "",
    "throw",
    "break",
    "",
    "",
    "",
    "",
    "",
    "",
    "extends",
    "",
    "accessor",
    "",
    "",
    "",
    "",
    "",
    "false",
    "",
    "",
    "",
    "",
    "",
    "",
    "delete",
    "",
    "",
    "",
    "",
    "",
    "",
    "import",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "super",
    "",
    "",
    "",
    "",
    "",
    "symbol",
    "",
    "",
    "",
    "",
    "string",
    "abstract",
    "",
    "async",
    "",
    "",
    "",
    "",
    "",
    "meta",
    "",
    "function",
    "for",
    "",
    "",
    "instanceof",
    "",
    "",
    "object",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "var",
    "require",
    "",
    "",
    "",
    "",
    "",
    "",
    "type",
    "",
    "as",
    "",
    "",
    "",
    "catch",
    "",
    "",
    "new",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "static",
    "",
    "true",
    "",
    "",
    "try",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "assert",
    "",
    "",
    "unknown",
    "debugger",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "if",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "this",
    "",
    "",
    "",
    "",
    "",
    "unique",
    "public",
    "",
    "",
    "",
    "",
    "",
    "any",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "of",
    "",
    "continue",
    "",
    "",
    "else",
    "",
    "",
    "",
    "",
    "",
    "implements",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "target",
    "",
    "",
    "",
    "",
    "",
    "boolean",
    "",
    "",
    "protected",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "declare",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "null",
    "",
    "",
    "",
    "",
    "",
    "",
    "is",
    "",
    "package",
    "",
    "return",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "module",
    "",
    "class",
    "",
    "from",
    "",
    "",
    "",
    "",
    "",
    "switch",
    "asserts",
    "",
    "",
    "",
    "satisfies",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
];

impl Cmov for Kind {
    fn cmovz(&mut self, value: Self, condition: cmov::Condition) {
        let mut tmp = *self as u8;
        tmp.cmovz(value as u8, condition);
        // SAFETY: Kind is represented as a single u8, and we know that the value is
        // always valid Kind variant
        *self = unsafe { std::mem::transmute(tmp) }
    }

    fn cmovnz(&mut self, _value: Self, _condition: cmov::Condition) {
        unimplemented!()
    }
}

#[inline]
pub fn table_match_keyword(s: &str) -> Kind {
    let slice = s.as_bytes();
    let extract = unsafe { hash::extract_first_and_last_two_bytes(slice) };
    let hash_code = hash::hash_u32(extract, HASH_TABLE_SEED);
    let idx = hash_code as usize % HASH_TABLE_SIZE;
    let mut kind = KIND_TABLE[idx];
    let key = STR_TABLE[idx];
    let condition = u8::from(s.len() == key.len() && s == key);
    kind.cmovz(Ident, condition);
    kind
}

#[cfg(test)]
#[test]
fn test_table_size() {
  assert_eq!(512, std::mem::size_of_val(&KIND_TABLE));
  assert_eq!(16 * 512, std::mem::size_of_val(&STR_TABLE));
}
