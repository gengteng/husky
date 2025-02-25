inductive PureBinaryOpr
  | Add
  | And
  | BitAnd
  | BitOr
  | BitXor
  | Div
  | Eq
  | Geq
  | Greater
  | Leq
  | Less
  | Mul
  | Neq
  | Or
  | RemEuclid
  | Power
  | Shl
  | Shr
  | Sub
  deriving DecidableEq

namespace PureBinaryOpr
def kindName : PureBinaryOpr -> String
  | Add => "Add"
  | And => "And"
  | BitAnd => "BitAnd"
  | BitOr => "BitOr"
  | BitXor => "BitXor"
  | Div => "Div"
  | Eq => "Eq"
  | Geq => "Geq"
  | Greater => "Greater"
  | Leq => "Leq"
  | Less => "Less"
  | Mul => "Mul"
  | Neq => "Neq"
  | Or => "Or"
  | RemEuclid => "RemEuclid"
  | Power => "Power"
  | Shl => "Shl"
  | Shr => "Shr"
  | Sub => "Sub"

def rustTraitMethodName : PureBinaryOpr -> String
  | Add => "add"
  | And => sorry
  | BitAnd => sorry
  | BitOr => sorry
  | BitXor => sorry
  | Div => sorry
  | Eq => "eq"
  | Geq => "ge"
  | Greater => "gt"
  | Leq => "le"
  | Less => "lt"
  | Mul => sorry
  | Neq => "ne"
  | RemEuclid => sorry
  | Or => sorry
  | Power => sorry
  | Shl => sorry
  | Shr => sorry
  | Sub => "sub"

def huskyCode : PureBinaryOpr -> String
  | Less => "<"
  | Leq => "<="
  | Greater => ">"
  | Geq => ">="
  | Neq => "!="
  | Eq => "=="
  | Shl => "<<"
  | Shr => ">>"
  | Add => "+"
  | Sub => "-"
  | Mul => "*"
  | Div => "/"
  | And => "&&"
  | BitAnd => "&"
  | Or => "||"
  | Power => "**"
  | BitXor => "^"
  | BitOr => "|"
  | RemEuclid => "%"

def spacedHuskyCode : PureBinaryOpr -> String
  | opr => s!" {opr.huskyCode} "
end PureBinaryOpr

inductive BinaryOpr
  | Pure : PureBinaryOpr -> BinaryOpr
  | Assign : Option PureBinaryOpr -> BinaryOpr