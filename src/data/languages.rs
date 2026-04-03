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
        label: "Assembler",
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

static ROWS: [IdiomRow; 11] = [
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
        id: "calling",
        label: "Calling a function",
        cells: &[
            ("apl", "n/a (deferred)"),
            ("a24", "JSR SQUARE"),
            ("basic", "GOSUB 100"),
            ("forth", "5 SQUARE"),
            ("fortran", "n/a"),
            ("lisp", "(sq 5)"),
            ("pascal", "Y := Sq(5);"),
            ("plsw", "CALL Square(5);"),
            ("sws", "n/a (v0.1)"),
        ],
    },
    IdiomRow {
        id: "comments",
        label: "Comments",
        cells: &[
            ("apl", ";; inline comment"),
            ("a24", "; comment"),
            ("basic", "REM this is a comment"),
            ("forth", "\\ line comment"),
            ("fortran", "n/a"),
            ("lisp", "; inline comment"),
            ("pascal", "{ comment }"),
            ("plsw", "/* comment */"),
            ("sws", "# comment"),
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
        id: "error-handling",
        label: "Error handling",
        cells: &[
            ("apl", "; error halts\n; REPL recovers"),
            ("a24", "; no runtime errors\n; undefined = 0"),
            ("basic", "; syntax error\n; line number shown"),
            ("forth", "CATCH THROW"),
            ("fortran", "n/a"),
            ("lisp", "; runtime error\n; error message shown"),
            ("pascal", "; runtime error\n; halted with message"),
            ("plsw", "ON ERROR GOTO lbl"),
            ("sws", "catch { script } err {\n  echo $err\n}"),
        ],
    },
    IdiomRow {
        id: "functions",
        label: "Functions",
        cells: &[
            ("apl", "n/a (deferred)"),
            ("a24", "n/a"),
            ("basic", "GOSUB 100\n  ... \n100 RETURN"),
            ("forth", ": SQUARE DUP * ;"),
            ("fortran", "n/a"),
            ("lisp", "(defun sq (x)\n  (* x x))"),
            ("pascal", "function Sq(X: integer)\n  : integer;"),
            ("plsw", "PROC Square;\n  DCL X INT;\n  ... END;"),
            ("sws", "n/a (v0.1)"),
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
    IdiomRow {
        id: "loops",
        label: "Loops",
        cells: &[
            ("apl", "LOOP: [] <- I\nI <- I - 1\ngoto (I)/LOOP"),
            (
                "a24",
                "LDA #0\n  STA I\nLP: OUT I\n  INC I\n  CMP #10\n  BNE LP",
            ),
            ("basic", "FOR I = 1 TO 10\n  PRINT I\nNEXT I"),
            ("forth", "10 0 DO I . LOOP"),
            ("fortran", "n/a"),
            ("lisp", "(dotimes (i 10)\n  (print i))"),
            ("pascal", "for I := 1 to 10 do\n  WriteLn(I);"),
            ("plsw", "DO I = 1 TO 10;\n  DISPLAY(I);\nEND;"),
            ("sws", "set i 0\nwhile {$i < 10} {\n  puts $i; incr i\n}"),
        ],
    },
    IdiomRow {
        id: "print",
        label: "Print",
        cells: &[
            ("apl", "[] <- 42"),
            ("a24", "LDA #42\n  OUT 0"),
            ("basic", "PRINT \"Hello\""),
            ("forth", ".\" Hello\" CR"),
            ("fortran", "n/a"),
            ("lisp", "(print 42)"),
            ("pascal", "WriteLn('Hello');"),
            ("plsw", "DISPLAY('Hello');"),
            ("sws", "echo Hello"),
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
        assert_eq!(rows().len(), 11);
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
