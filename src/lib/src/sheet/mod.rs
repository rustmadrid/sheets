
/// The position of a cell in a spreadsheet. The cell at A1 has `Coord(0, 0)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coord(pub usize, pub usize);

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
}
