use crate::Pg;

sqlwright_core::sql_types!(Pg {
    Numeric => "numeric",

    Money => "money",

    VarChar => "character varying",
    Char => "character",

    Bytea => "bytea",

    Timestamp => "timestamp",
    Timestamptz => "timestamp with time zone",
    Date => "date",
    Time => "time",
    Timetz => "time with time zone",
    Interval => "interval",

    Point => "point",
    Line => "line",
    LSeg => "lseg",
    PgBox => "box",
    Path => "path",
    Polygon => "polygon",
    Circle => "circle",

    Inet => "inet",
    Cidr => "cidr",
    MacAddr => "macaddr",
    MacAddr8 => "macaddr8",

    Bit => "bit",
    VarBit => "bit varying",

    TsVector => "tsvector",
    TsQuery => "tsquery",

    Uuid => "uuid",

    Xml => "xml",

    Json => "json",
    Jsonb => "jsonb",
    JsonPath => "jsonpath",

    Int4Range => "int4range",
    Int8Range => "int8range",
    NumRange => "numrange",
    TsRange => "tsrange",
    TsTzRange => "tstzrange",
    DateRange => "daterange",
    Int4MultiRange => "int4multirange",
    Int8MultiRange => "int8multirange",
    NumMultiRange => "nummultirange",
    TsMultiRange => "tsmultirange",
    TsTzMultiRange => "tstzmultirange",
    DateMultiRange => "datemultirange",

    Oid => "oid",
    RegClass => "regclass",
    RegCollation => "regcollation",
    RegConfig => "regconfig",
    RegDictionary => "regdictionary",
    RegNamespace => "regnamespace",
    RegOper => "regoper",
    RegOperator => "regoperator",
    RegProc => "regproc",
    RegProcedure => "regprocedure",
    RegRole => "regrole",
    RegType => "regtype",

    PgLsn => "pg_lsn",
});
