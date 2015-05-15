use ::std::collections::{HashMap, HashSet};

pub struct Sheet {
    cells: HashMap<Coord, Formula>,
}

impl Sheet {
    pub fn new() -> Self {
        Sheet{cells: HashMap::new()}
    }

    pub fn set(&mut self, coord: Coord, formula: Formula) {
        if let Formula::Atom(FormulaAtom::Empty) = formula {
            self.cells.remove(&coord);
        } else {
            self.cells.insert(coord, formula);
        };
    }

    pub fn value(&self, coord: Coord) -> Result<Box<FormulaAtom>, FormulaErr> {
        match self.cells.get(&coord) {
            Some(x) => self.calc_formula(x),
            _ => Ok(Box::new(FormulaAtom::Empty)),
        }
    }

    fn calc_formula(&self, formula: &Formula) -> Result<Box<FormulaAtom>, FormulaErr> {
        let mut visited = HashSet::new();
        self.calc_formula_visited(formula, &mut visited)
    }

    fn calc_formula_visited(&self, formula: &Formula, visited: &mut HashSet<Coord>) -> Result<Box<FormulaAtom>, FormulaErr> {
        match *formula {
            Formula::Atom(ref x) => Ok(Box::new(x.clone())),
            Formula::Ref(coord) => {
                if visited.contains(&coord) {
                    return Err(FormulaErr::Ref(coord));
                }
                visited.insert(coord);
                match self.cells.get(&coord) {
                    Some(f) => self.calc_formula_visited(f, visited),
                    None => Ok(Box::new(FormulaAtom::Empty)),
                }
            },
            Formula::Op(ref op, ref args) => {
                let mut atoms = Vec::with_capacity(args.len());
                for arg in args {
                    match self.calc_formula_visited(&arg, &mut visited.clone()) {
                        Ok(x) => { atoms.push(x); }
                        Err(x) => { return Err(x); },
                    }
                }

                match *op {
                    FormulaOp::Add => {
                        use ::std::ops::Add;
                        self.numeric_op(Add::add, &atoms)
                    },
                    FormulaOp::Sub => {
                        use ::std::ops::Sub;
                        self.numeric_op(Sub::sub, &atoms)
                    },
                    FormulaOp::Mul => {
                        use ::std::ops::Mul;
                        self.numeric_op(Mul::mul, &atoms)
                    },
                    FormulaOp::Div => {
                        use ::std::ops::Div;
                        self.numeric_op(Div::div, &atoms)
                    },
                    FormulaOp::Avg => {
                        use ::std::ops::Add;
                        match self.numeric_op(Add::add, &atoms) {
                            Ok(b) => {
                                if let FormulaAtom::Number(ref x) = *b {
                                    Ok(Box::new(FormulaAtom::Number(x / atoms.len() as f64)))
                                } else {
                                    unreachable!()
                                }
                            },
                            Err(x) => Err(x)
                        }
                    },
                }
            }
        }
    }

    fn numeric_op<F>(&self, f: F, atoms: &Vec<Box<FormulaAtom>>) -> Result<Box<FormulaAtom>, FormulaErr>
        where F: Fn(f64, f64) -> f64
    {
        if atoms.len() == 0 {
            return Err(FormulaErr::Arity(1));
        }

        let mut first = true;
        let mut ret = 0.0;
        for a in atoms {
            match **a {
                FormulaAtom::Number(x) => { 
                    if first {
                        ret = x;
                        first = false;
                    } else {
                        ret = f(ret, x);
                    }
                },
                _ => { return Err(FormulaErr::Type("Number")) }
            }
        }
        Ok(Box::new(FormulaAtom::Number(ret)))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Formula {
    Atom(FormulaAtom),
    Ref(Coord),
    Op(FormulaOp, Vec<Formula>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum FormulaAtom {
    Empty,
    String(String),
    Number(f64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum FormulaOp {
    Add,
    Sub,
    Mul,
    Div,
    Avg,
}

pub enum FormulaErr {
    Ref(Coord),
    Type(&'static str),
    Arity(u8),
}

/// The position of a cell in a spreadsheet. The cell at A1 has `Coord(0, 0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub usize, pub usize);

impl Coord {
    pub fn parse(s: &str) -> Option<Self> {
        let idx = match s.find(|c: char| c.is_numeric()) {
            None => { return None; },
            Some(x) => x,
        };

        let (l, r) = (&s[0..idx], &s[idx..]);

        match r.parse().ok() {
            None => None,
            Some(x) => Some(natural_to_numeric(l, x)),
        }
    }

    pub fn format_natural(&self) -> String {
        let Coord(col, row) = *self;
        let mut ret = numeric_col_to_natural(col);
        ret.push_str(format!("{}", row + 1).as_str());
        ret
    }
}

/// The position of a cell in a spreadsheet in a "natural" representation.
/// The cell at A1 has `Coord("A1", 1)`.
#[derive(Debug, PartialEq,  Eq)]
pub struct NatCoord<'a>(pub &'a str, pub usize);

impl<'a> NatCoord<'a> {
    pub fn to_numeric(&'a self) -> Coord {
        let NatCoord(col, row) = *self;
        natural_to_numeric(col, row)
    }
}

/// Converts the coordinates in "natural" components (see `NatCoord`).
///
/// # Examples
/// 
/// ```
/// use sheets_lib::sheet::*;
///
/// assert_eq!(Coord(35, 3), natural_to_numeric("AJ", 4))
/// ```
pub fn natural_to_numeric(col: &str, row: usize) -> Coord {
    assert!(row >= 1);
    Coord(natural_col_to_numeric(col), row - 1)
}

pub fn natural_col_to_numeric(col: &str) -> usize {
    assert!(col.len() > 0);

    let mut pow = 1;
    col.bytes().rev().fold(0, |acc, item| {
        assert!(item >= 'A' as u8 && item <= 'Z' as u8);
        let letter_val = (item - 'A' as u8) as usize + 1;
        let ret = acc + pow * letter_val; 
        pow *= 26;
        ret
    }) - 1
}

pub fn numeric_col_to_natural(col: usize) -> String {
    // TODO: This is only for two letters.
    let mut ret = "".to_string();
    let c = col + 1;
    let i = c / 27;
    let rem = col - (i * 26);
    if i > 0 {
        ret.push((i as u8 + 'A' as u8) as char);
    }
    if rem > 0 {
        ret.push((rem as u8 + 'A' as u8) as char);
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_natural_to_numeric() {
        assert_eq!(Coord(0, 0), natural_to_numeric("A", 1));
        assert_eq!(Coord(1, 0), natural_to_numeric("B", 1));
        assert_eq!(Coord(25, 9), natural_to_numeric("Z", 10));
        assert_eq!(Coord(26, 9), natural_to_numeric("AA", 10));
        assert_eq!(Coord(27, 9), natural_to_numeric("AB", 10));
        assert_eq!(Coord(26 + 25, 9), natural_to_numeric("AZ", 10));
        assert_eq!(Coord(26 + 26, 9), natural_to_numeric("BA", 10));
        assert_eq!(Coord(701, 9), natural_to_numeric("ZZ", 10));
        assert_eq!(Coord(702, 9), natural_to_numeric("AAA", 10));
    }

    #[test]
    #[should_panic]
    fn test_bad_natural_col() {
        natural_to_numeric("AÑB", 1);
    }

    #[test]
    #[should_panic]
    fn test_empty_natural_col() {
        natural_to_numeric("", 1);
    }

    #[test]
    #[should_panic]
    fn test_bad_natural_row() {
        natural_to_numeric("A", 0);
    }

    #[test]
    fn test_sheet() {
        let mut sheet = Sheet::new();
        assert_eq!(FormulaAtom::Empty, *sheet.value(Coord(0, 0)).ok().unwrap());
        assert_eq!(FormulaAtom::Empty, *sheet.value(Coord(2, 3)).ok().unwrap());

        sheet.set(Coord(2, 3), Formula::Atom(FormulaAtom::Empty));
        assert_eq!(FormulaAtom::Empty, *sheet.value(Coord(0, 0)).ok().unwrap());
        assert_eq!(FormulaAtom::Empty, *sheet.value(Coord(2, 3)).ok().unwrap());

        sheet.set(Coord(2, 3), Formula::Atom(FormulaAtom::String("test".to_string())));
        assert_eq!(FormulaAtom::Empty, *sheet.value(Coord(0, 0)).ok().unwrap());
        if let FormulaAtom::String(ref x) = *sheet.value(Coord(2, 3)).ok().unwrap() {
            assert_eq!("test", *x);
        } else { panic!(); };

        sheet.set(Coord(3, 4), Formula::Ref(Coord(2, 3)));
        sheet.set(Coord(5, 6), Formula::Ref(Coord(3, 4)));
        if let FormulaAtom::String(ref x) = *sheet.value(Coord(5, 6)).ok().unwrap() {
            assert_eq!("test", *x);
        } else { panic!(); };
    }

    #[test]
    fn test_sheet_cycle() {
        let mut sheet = Sheet::new();
        sheet.set(Coord(0, 0), Formula::Ref(Coord(0, 0)));
        if let Err(FormulaErr::Ref(x)) = sheet.value(Coord(0, 0)) {
            assert_eq!(Coord(0, 0), x);
        } else { panic!(); };
    }

    #[test]
    fn test_sheet_long_cycle() {
        let mut sheet = Sheet::new();
        sheet.set(Coord(0, 0), Formula::Ref(Coord(0, 1)));
        sheet.set(Coord(0, 1), Formula::Ref(Coord(0, 2)));
        sheet.set(Coord(0, 2), Formula::Ref(Coord(0, 3)));
        sheet.set(Coord(0, 3), Formula::Ref(Coord(0, 1)));
        if let Err(FormulaErr::Ref(x)) = sheet.value(Coord(0, 0)) {
            assert_eq!(Coord(0, 1), x);
        } else { panic!(); };
    }

    #[test]
    fn test_sheet_add() {
        let mut sheet = Sheet::new();
        sheet.set(Coord(0, 0), Formula::Atom(FormulaAtom::Number(2.0)));
        sheet.set(Coord(0, 1), Formula::Atom(FormulaAtom::Number(3.0)));
        sheet.set(Coord(0, 2), Formula::Op(
            FormulaOp::Add,
            vec![
                Formula::Atom(FormulaAtom::Number(4.0)),
                Formula::Ref(Coord(0, 0)),
                Formula::Ref(Coord(0, 1)),
            ]));

        assert_eq!(FormulaAtom::Number(9.0), *sheet.value(Coord(0, 2)).ok().unwrap());
    }

    #[test]
    fn test_sheet_add_cycle() {
        let mut sheet = Sheet::new();
        sheet.set(Coord(0, 0), Formula::Atom(FormulaAtom::Number(2.0)));
        sheet.set(Coord(0, 1), Formula::Ref(Coord(0, 1)));
        sheet.set(Coord(0, 2), Formula::Op(
            FormulaOp::Add,
            vec![
                Formula::Atom(FormulaAtom::Number(4.0)),
                Formula::Ref(Coord(0, 0)),
                Formula::Ref(Coord(0, 1)),
            ]));

        if let Err(FormulaErr::Ref(x)) = sheet.value(Coord(0, 2)) {
            assert_eq!(Coord(0, 1), x);
        } else { panic!(); };
    }

    #[test]
    fn test_sheet_add_string() {
        let mut sheet = Sheet::new();
        sheet.set(Coord(0, 0), Formula::Atom(FormulaAtom::Number(2.0)));
        sheet.set(Coord(0, 1), Formula::Atom(FormulaAtom::String("not a number".to_string())));
        sheet.set(Coord(0, 2), Formula::Op(
            FormulaOp::Add,
            vec![
                Formula::Atom(FormulaAtom::Number(4.0)),
                Formula::Ref(Coord(0, 0)),
                Formula::Ref(Coord(0, 1)),
            ]));

        if let Err(FormulaErr::Type(x)) = sheet.value(Coord(0, 2)) {
            assert_eq!("Number", x);
        } else { panic!(); };
    }

    #[test]
    fn test_sheet_avg() {
        let mut sheet = Sheet::new();
        sheet.set(Coord(0, 0), Formula::Atom(FormulaAtom::Number(2.0)));
        sheet.set(Coord(0, 1), Formula::Atom(FormulaAtom::Number(3.0)));
        sheet.set(Coord(0, 2), Formula::Op(
            FormulaOp::Avg,
            vec![
                Formula::Atom(FormulaAtom::Number(10.0)),
                Formula::Ref(Coord(0, 0)),
                Formula::Ref(Coord(0, 1)),
            ]));

        assert_eq!(FormulaAtom::Number(5.0), *sheet.value(Coord(0, 2)).ok().unwrap());
    }
}
