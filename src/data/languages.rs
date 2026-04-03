#[derive(Clone, PartialEq, Debug)]
pub struct LangColumn {
    pub id: &'static str,
    pub label: &'static str,
    pub default_collapsed: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct IdiomRow {
    pub id: &'static str,
    pub label: &'static str,
    pub cells: &'static [(&'static str, &'static str)],
}

pub fn columns() -> &'static [LangColumn] {
    &COLUMNS
}

pub fn rows() -> &'static [IdiomRow] {
    &ROWS
}

pub fn cell_value(row: &IdiomRow, col_id: &str) -> &'static str {
    for (cid, val) in row.cells {
        if *cid == col_id {
            return val;
        }
    }
    "n/a"
}

static COLUMNS: [LangColumn; 9] = [
    LangColumn {
        id: "apl",
        label: "APL",
        default_collapsed: false,
    },
    LangColumn {
        id: "a24",
        label: "a24",
        default_collapsed: false,
    },
    LangColumn {
        id: "basic",
        label: "BASIC",
        default_collapsed: false,
    },
    LangColumn {
        id: "forth",
        label: "Forth",
        default_collapsed: false,
    },
    LangColumn {
        id: "fortran",
        label: "Fortran",
        default_collapsed: true,
    },
    LangColumn {
        id: "lisp",
        label: "Lisp",
        default_collapsed: false,
    },
    LangColumn {
        id: "pascal",
        label: "Pascal",
        default_collapsed: false,
    },
    LangColumn {
        id: "plsw",
        label: "PL/SW",
        default_collapsed: false,
    },
    LangColumn {
        id: "sws",
        label: "SWS",
        default_collapsed: false,
    },
];

static ROWS: [IdiomRow; 5] = [
    IdiomRow {
        id: "arithmetic",
        label: "Arithmetic",
        cells: &[
            ("apl", "X + Y \u{00d7} Z"),
            ("a24", "LDA A, X\n  ADD A, Y\n  MUL B, Z\n  STA X"),
            ("basic", "X = A + B * C"),
            ("forth", "X Y + Z *"),
            ("fortran", "n/a"),
            ("lisp", "(+ X (* Y Z))"),
            ("pascal", "X := A + B * C"),
            ("plsw", "X = A + B * C"),
            ("sws", "set X [expr {$A + $B * $C}]"),
        ],
    },
    IdiomRow {
        id: "assignment",
        label: "Assignment",
        cells: &[
            ("apl", "X \u{2190} 42"),
            ("a24", "LDA #42\n  STA X"),
            ("basic", "LET X = 42"),
            ("forth", "42 VARIABLE X"),
            ("fortran", "n/a"),
            ("lisp", "(SETQ X 42)"),
            ("pascal", "X := 42"),
            ("plsw", "X = 42"),
            ("sws", "set X 42"),
        ],
    },
    IdiomRow {
        id: "booleans",
        label: "Booleans",
        cells: &[
            ("apl", "X > 0"),
            ("a24", "LDA X\n  CMP #0"),
            ("basic", "X > 0"),
            ("forth", "X 0 >"),
            ("fortran", "n/a"),
            ("lisp", "(> X 0)"),
            ("pascal", "X > 0"),
            ("plsw", "X > 0"),
            ("sws", "expr {$X > 0}"),
        ],
    },
    IdiomRow {
        id: "conditionals",
        label: "Conditionals",
        cells: &[
            ("apl", "\u{2192}(X>0) 'yes' 'no'"),
            ("a24", "LDA X\n  CMP #0\n  BNE yes\n  ; no path"),
            ("basic", "IF X > 0 THEN PRINT \"yes\""),
            ("forth", "X 0 > IF CR .\" yes\" THEN"),
            ("fortran", "n/a"),
            ("lisp", "(IF (> X 0) \"yes\" \"no\")"),
            ("pascal", "IF X > 0 THEN\n  WriteLn('yes');"),
            ("plsw", "IF X > 0 THEN\n  DISPLAY('yes');"),
            ("sws", "if {$X > 0} {puts yes}"),
        ],
    },
    IdiomRow {
        id: "io",
        label: "I/O (switch, LED)",
        cells: &[
            ("apl", "'MMIO' SVO 242\nSW <- MMIO[0]\nMMIO[0] <- LED"),
            ("a24", "IN 1\n  CMP #1\n  BNE skip\n  LDA #1\n  OUT 2"),
            ("basic", "IF IN(1) = 1 THEN OUT 2, 1"),
            ("forth", "IN@ IF 2 OUT@ THEN"),
            ("fortran", "n/a"),
            ("lisp", "(if (s2-pressed?)\n  (begin (set-leds 0)...))"),
            ("pascal", "Port[1] := Port[2]"),
            ("plsw", "n/a"),
            ("sws", "set led [expr {[read switch]}]"),
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_count() {
        assert_eq!(columns().len(), 9);
    }

    #[test]
    fn row_count() {
        assert_eq!(rows().len(), 5);
    }

    #[test]
    fn all_rows_have_all_columns() {
        let col_ids: Vec<&str> = columns().iter().map(|c| c.id).collect();
        for row in rows() {
            for cid in &col_ids {
                let val = cell_value(row, cid);
                assert!(!val.is_empty(), "empty cell for {} in {}", cid, row.id);
            }
        }
    }

    #[test]
    fn fortran_default_collapsed() {
        let fortran = columns().iter().find(|c| c.id == "fortran").unwrap();
        assert!(fortran.default_collapsed);
    }

    #[test]
    fn rows_alphabetical() {
        let labels: Vec<&str> = rows().iter().map(|r| r.label).collect();
        let mut sorted = labels.clone();
        sorted.sort();
        assert_eq!(labels, sorted, "rows are not in alphabetical order");
    }
}
