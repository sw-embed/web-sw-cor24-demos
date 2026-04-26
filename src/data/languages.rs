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

static COLUMNS: [LangColumn; 15] = [
    LangColumn {
        id: "a24",
        label: "a24-sw",
        default_collapsed: false,
    },
    LangColumn {
        id: "apl",
        label: "apl-sw",
        default_collapsed: false,
    },
    LangColumn {
        id: "basic",
        label: "basic-sw",
        default_collapsed: false,
    },
    LangColumn {
        id: "forth",
        label: "forth-sw",
        default_collapsed: false,
    },
    LangColumn {
        id: "fortran",
        label: "fortran-sw",
        default_collapsed: true,
    },
    LangColumn {
        id: "lisp",
        label: "macrolisp-sw",
        default_collapsed: false,
    },
    LangColumn {
        id: "ocaml",
        label: "ocaml-sw",
        default_collapsed: true,
    },
    LangColumn {
        id: "pascal",
        label: "pascal-sw",
        default_collapsed: false,
    },
    LangColumn {
        id: "plsw",
        label: "PL/SW",
        default_collapsed: false,
    },
    LangColumn {
        id: "prolog",
        label: "prolog-sw",
        default_collapsed: true,
    },
    LangColumn {
        id: "rpg-ii",
        label: "rpg-ii-sw",
        default_collapsed: true,
    },
    LangColumn {
        id: "smalltalk",
        label: "smalltalk-sw",
        default_collapsed: true,
    },
    LangColumn {
        id: "snobol4",
        label: "SNOBOL4",
        default_collapsed: false,
    },
    LangColumn {
        id: "sws",
        label: "sws",
        default_collapsed: false,
    },
    LangColumn {
        id: "tuplet",
        label: "tuplet",
        default_collapsed: true,
    },
];

static ROWS: [IdiomRow; 11] = [
    IdiomRow {
        id: "arithmetic",
        label: "Arithmetic",
        cells: &[
            ("apl", "X + Y * Z"),
            ("a24", "LDA A, X\n  ADD A, Y\n  MUL B, Z\n  STA X"),
            ("basic", "X = A + B * C"),
            ("forth", "X Y + Z *"),
            ("fortran", "n/a"),
            ("lisp", "(+ X (* Y Z))"),
            ("ocaml", "n/a"),
            ("pascal", "X := A + B * C"),
            ("plsw", "X = A + B * C"),
            ("prolog", "n/a"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "X := A + (B * C)"),
            ("snobol4", "Y = X * 6"),
            ("sws", "set X 42"),
            ("tuplet", "user-minted"),
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
            ("ocaml", "n/a"),
            ("pascal", "X > 0"),
            ("plsw", "X > 0"),
            ("prolog", "X > 0"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "X > 0"),
            ("snobol4", "n/a (v0.1)"),
            ("sws", "expr {$X > 0}"),
            ("tuplet", "user-minted"),
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
            ("ocaml", "n/a"),
            ("pascal", "Y := Sq(5);"),
            ("plsw", "CALL Square(5);"),
            ("prolog", "square(5, Y)."),
            ("rpg-ii", "n/a"),
            ("smalltalk", "5 squared"),
            ("snobol4", "n/a (v0.1)"),
            ("sws", "n/a (v0.1)"),
            ("tuplet", "user-minted"),
        ],
    },
    IdiomRow {
        id: "comments",
        label: "Comments",
        cells: &[
            ("apl", "comment this is text"),
            ("a24", "; comment"),
            ("basic", "REM this is a comment"),
            ("forth", "\\ line comment"),
            ("fortran", "n/a"),
            ("lisp", "; inline comment"),
            ("ocaml", "(* comment *)"),
            ("pascal", "{ comment }"),
            ("plsw", "/* comment */"),
            ("prolog", "% comment"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "\"comment\""),
            ("snobol4", "* comment"),
            ("sws", "# comment"),
            ("tuplet", "user-minted"),
        ],
    },
    IdiomRow {
        id: "conditionals",
        label: "Conditionals",
        cells: &[
            ("apl", "goto (X > 0) / 'yes' 'no'"),
            ("a24", "LDA X\n  CMP #0\n  BNE yes\n  ; no path"),
            ("basic", "IF X > 0 THEN PRINT \"yes\""),
            ("forth", "X 0 > IF CR .\" yes\" THEN"),
            ("fortran", "n/a"),
            ("lisp", "(IF (> X 0) \"yes\" \"no\")"),
            ("ocaml", "n/a"),
            ("pascal", "IF X > 0 THEN\n  WriteLn('yes');"),
            ("plsw", "IF X > 0 THEN\n  DISPLAY('yes');"),
            ("prolog", "X > 0 -> write(yes)"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "(X > 0)\n  ifTrue: [Transcript show: 'yes']"),
            ("snobol4", "TEXT PAT . N :F(NO)"),
            ("sws", "if {$X > 0} {puts yes}"),
            ("tuplet", "essential special form\n+ user-minted macros"),
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
            ("ocaml", "n/a"),
            ("pascal", "; runtime error\n; halted with message"),
            ("plsw", "ON ERROR GOTO lbl"),
            ("prolog", "n/a"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "[block]\n  on: Error do: [:e | ...]"),
            ("snobol4", ":F(NO)"),
            ("sws", "catch { script } err {\n  echo $err\n}"),
            ("tuplet", "user-minted"),
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
            ("ocaml", "n/a"),
            ("pascal", "function Sq(X: integer)\n  : integer;"),
            ("plsw", "PROC Square;\n  DCL X INT;\n  ... END;"),
            ("prolog", "square(X, Y) :-\n  Y is X * X."),
            ("rpg-ii", "n/a"),
            ("smalltalk", "sq := [:x | x * x]"),
            ("snobol4", "n/a (v0.1)"),
            ("sws", "n/a (v0.1)"),
            (
                "tuplet",
                "user-minted (\"verb\")\n2D layout, whitespace-significant",
            ),
        ],
    },
    IdiomRow {
        id: "io",
        label: "I/O (switch, LED)",
        cells: &[
            (
                "apl",
                "'MMIO' qsvo 242\nSW assign MMIO[0]\nMMIO[0] assign LED",
            ),
            ("a24", "IN 1\n  CMP #1\n  BNE skip\n  LDA #1\n  OUT 2"),
            ("basic", "IF IN(1) = 1 THEN OUT 2, 1"),
            ("forth", "IN@ IF 2 OUT@ THEN"),
            ("fortran", "n/a"),
            ("lisp", "(if (s2-pressed?)\n  (begin (set-leds 0)...))"),
            ("ocaml", "n/a"),
            ("pascal", "Port[1] := Port[2]"),
            ("plsw", "n/a"),
            ("prolog", "n/a"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "Switch read\nLED show: 1"),
            ("snobol4", "LINE = INPUT"),
            ("sws", "set led [expr {[read switch]}]"),
            ("tuplet", "user-minted"),
        ],
    },
    IdiomRow {
        id: "loops",
        label: "Loops",
        cells: &[
            ("apl", "LOOP: [] assign I\nI assign I - 1\ngoto (I)/LOOP"),
            (
                "a24",
                "LDA #0\n  STA I\nLP: OUT I\n  INC I\n  CMP #10\n  BNE LP",
            ),
            ("basic", "FOR I = 1 TO 10\n  PRINT I\nNEXT I"),
            ("forth", "10 0 DO I . LOOP"),
            ("fortran", "n/a"),
            ("lisp", "(dotimes (i 10)\n  (print i))"),
            ("ocaml", "n/a"),
            ("pascal", "for I := 1 to 10 do\n  WriteLn(I);"),
            ("plsw", "DO I = 1 TO 10;\n  DISPLAY(I);\nEND;"),
            ("prolog", "between(1, 10, I)"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "1 to: 10 do: [:i |\n  Transcript show: i]"),
            ("snobol4", ":(READ)"),
            ("sws", "set i 0\nwhile {$i < 10} {\n  puts $i; incr i\n}"),
            ("tuplet", "user-minted (macro)"),
        ],
    },
    IdiomRow {
        id: "pattern-match",
        label: "Pattern matching",
        cells: &[
            ("apl", "X match /\\d+/"),
            ("a24", "n/a"),
            ("basic", "n/a"),
            ("forth", "n/a"),
            ("fortran", "n/a"),
            ("lisp", "(scan-to-regex s)"),
            ("ocaml", "n/a"),
            ("pascal", "n/a"),
            ("plsw", "n/a"),
            ("prolog", "append([1,2], [3], X)"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "n/a"),
            ("snobol4", "SPAN('0123456789')"),
            ("sws", "regexp {\\d+}"),
            ("tuplet", "user-minted (named tuples)"),
        ],
    },
    IdiomRow {
        id: "print",
        label: "Print",
        cells: &[
            ("apl", "quad assign 42"),
            ("a24", "LDA #42\n  OUT 0"),
            ("basic", "PRINT \"Hello\""),
            ("forth", ".\" Hello\" CR"),
            ("fortran", "n/a"),
            ("lisp", "(print 42)"),
            ("ocaml", "n/a"),
            ("pascal", "WriteLn('Hello');"),
            ("plsw", "DISPLAY('Hello');"),
            ("prolog", "write('Hello')"),
            ("rpg-ii", "n/a"),
            ("smalltalk", "Transcript show: 'Hello'"),
            ("snobol4", "OUTPUT = 'Hello'"),
            ("sws", "echo Hello"),
            ("tuplet", "user-minted"),
        ],
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn column_count() {
        assert_eq!(columns().len(), 15);
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

    #[test]
    fn columns_alphabetical() {
        let labels: Vec<&str> = columns().iter().map(|c| c.label).collect();
        let mut sorted = labels.clone();
        sorted.sort_by_key(|a| a.to_lowercase());
        assert_eq!(labels, sorted, "columns are not in alphabetical order");
    }
}
