#![plugin(peg_syntax_ext)]

use sheet::{Formula, FormulaAtom};

peg! grammar(r#"
use sheet::{Formula, FormulaAtom, FormulaOp, Coord};

#[pub]
formula -> Formula
    = string
    / number
    / ref
    / op

string -> Formula
    = "\"" s:inside_str "\"" { s }

inside_str -> Formula
    = [^"]* { Formula::Atom(FormulaAtom::String(match_str.to_string())) }

number -> Formula
    = "-"? [0-9]* "." [0-9]+ { Formula::Atom(FormulaAtom::Number(match_str.parse().unwrap())) }

ref -> Formula
    = [A-Z]+[1-9][0-9]* { Formula::Ref(Coord::parse(match_str).unwrap()) }

op -> Formula
    = o:op_name "(" args:formula ** arg_delim ")" {
        Formula::Op(o, args)
    }

arg_delim -> ()
    = "," [ \t]* { }

op_name -> FormulaOp
    = "add" { FormulaOp::Add }
    / "sub" { FormulaOp::Sub }
    / "mul" { FormulaOp::Div }
    / "div" { FormulaOp::Div }
    / "avg" { FormulaOp::Avg }
"#);

pub fn parse_formula(s: &str) -> Result<Formula, grammar::ParseError> {
    if s.len() == 0 {
        Ok(Formula::Atom(FormulaAtom::Empty))
    } else {
        grammar::formula(s)
    }
}

pub fn format_formula(f: &Formula) -> String {
    use sheet::{FormulaAtom, FormulaOp};

    match *f {
        Formula::Atom(FormulaAtom::Number(ref x)) => format!("{}", *x),
        Formula::Atom(FormulaAtom::String(ref x)) => format!("\"{}\"", *x),
        Formula::Atom(FormulaAtom::Empty) => "".to_string(),
        Formula::Ref(ref coord) => coord.format_natural(),
        Formula::Op(ref op, ref args) => {
            let mut ret = match *op {
                FormulaOp::Add => "add",
                FormulaOp::Sub => "sub",
                FormulaOp::Mul => "mul",
                FormulaOp::Div => "div",
                FormulaOp::Avg => "avg",
            }.to_string();
            ret.push_str("(");
            let first = true;
            for arg in args {
                if !first {
                    ret.push_str(", ");
                }
                ret.push_str(format_formula(arg).as_str());
            }
            ret.push_str(")");
            ret
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sheet::{Formula, Sheet, FormulaAtom, FormulaOp, Coord};
    
    #[test]
    fn test_string() {
        let r = parse_formula("\"foo\"").ok().unwrap();
        if let Formula::Atom(FormulaAtom::String(ref x)) = r {
            assert_eq!("foo", *x);
            assert_eq!("\"foo\"", format_formula(&r).as_str());
        } else { panic!(); };
    }

    #[test]
    fn test_number() {
        let mut r = parse_formula("123.4").ok().unwrap();
        if let Formula::Atom(FormulaAtom::Number(ref x)) = r {
            assert_eq!(123.4, *x);
            assert_eq!("123.4", format_formula(&r).as_str());
        } else { panic!(); };

        r = parse_formula("-0.34").ok().unwrap();
        if let Formula::Atom(FormulaAtom::Number(ref x)) = r {
            assert_eq!(-0.34, *x);
            assert_eq!("-0.34", format_formula(&r).as_str());
        } else { panic!(); };
    }

    #[test]
    fn test_ref() {
        let mut r = parse_formula("C4").ok().unwrap();
        assert_eq!(Formula::Ref(Coord(2, 3)), r);
        assert_eq!("C4", format_formula(&r).as_str());

        r = parse_formula("AAB45").ok().unwrap();
        assert_eq!(Formula::Ref(Coord(703, 44)), r);
        // assert_eq!("AAB45", format_formula(&r).as_str());
    }

    #[test]
    fn test_op() {
        let formula = parse_formula("add(1.0, sub(A3, 3.0), 4.6)").ok().unwrap();
        if let Formula::Op(FormulaOp::Add, _) = formula {
            let mut sheet = Sheet::new();
            sheet.set(Coord(0, 2), Formula::Atom(FormulaAtom::Number(15.0)));
            sheet.set(Coord(5, 5), formula);
            if let FormulaAtom::Number(ref x) = *sheet.value(Coord(5, 5)).ok().unwrap() {
                assert_eq!(17.6, *x);
            } else { panic!(); }
        } else { panic!(); };
    }
}
