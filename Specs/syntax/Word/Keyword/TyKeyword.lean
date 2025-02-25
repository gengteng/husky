inductive TyKeyword
  | Struct
  | Enum
  | Record
  deriving DecidableEq

namespace TyKeyword
def toRustVersion : TyKeyword -> String
  | Struct => "TyKeyword::Struct"
  | Enum => "TyKeyword::Enum"
  | Record => "TyKeyword::Record"

instance : ToString TyKeyword where
  toString : TyKeyword -> String
  | Struct => "struct"
  | Enum => "enum"
  | Record => "record"
end TyKeyword

-- namespace TyKeyword
-- def TyKeywordEnumeration := [
--   Struct,
--   Enum,
--   Record
-- ]
-- def as_str (kw : TyKeyword) : String :=
--   match kw with
--   | Struct => "struct"
--   | Enum => "enum"
--   | Record => "record"
-- end TyKeyword