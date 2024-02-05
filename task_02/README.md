
# Übungsblatt 2
## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, mit null Punkten bewertet werden! Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen. Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.

## Abgabemodus
Sie können in ihrer Lösung beliebige Hilfstypen und Funktionen selbst definieren. Die Einbindung von externen Crates und zusätzlichen Modulen ist jedoch nicht zulässig. Ebenso dürfen grundlegende Struktur des hier mitgelieferten Templates nicht verändert werden. Wichtig ist ebenfalls, dass definierte Namen, Signaturen von Methoden und absolute Pfade nicht verändert werden dürfen. Die Teile der Dateien, die von Ihnen bearbeitet werden sollen, sind grundsätzlich mit dem Schlagwort `TODO` gekennzeichnet.

Mögliche Aufrufe des Programms:
- Sie können die Implementation mit `cargo run -- <param>` aufrufen. Für den Platzhalter `<param>` können sie eigene Dateipfade.
- Mit dem Aufruf `cargo test` werden die Testfälle gestartet.
- Mit `cargo test -- --nocapture` werden auch Konsolenausgaben bei der Ausführung der Tests angezeigt.

## Enthaltenen Dateien
Muss bearbeitet und abgegeben werden:
- `src/c1_lex.rs`

## Aufgabe 1 (50 Punkte)
### Lernziel
In diesem Übungsblatt sollen Sie die erste Grundlage eines Compilers schaffen. Dazu setzen Sie sich mit dem Konzept des Lexers auseinander.

### Kurzbeschreibung (30 Punkte)
Implementieren Sie mithilfe von [logos](https://crates.io/crates/logos) einen Scanner, der in einem Eingabestrom bzw. in einer Eingabedatei die Token der Sprache C1 erkennt.

### Aufgabenstellung
Mittels logos sollen Sie einen Scanner implementieren, der aus einem Eingabestrom die Token der Sprache C1 extrahiert. Verfollständigen Sie dazu die Datei [c1_lexer.rs](src/c1_lexer.rs).

Die Lexik der Sprache C1 ist hier aufgelistet:
```
-- Schlüsselwörter
KW_BOOLEAN         "bool"
KW_DO              "do"
KW_ELSE            "else"
KW_FLOAT           "float"
KW_FOR             "for"
KW_IF              "if"
KW_INT             "int"
KW_PRINTF          "printf"
KW_RETURN          "return"
KW_VOID            "void"
KW_WHILE           "while"
-- Operatoren
PLUS               "+"
MINUS              "-"
ASTERISK           "*"
SLASH              "/"
ASSIGN             "="
EQ                 "=="
NEQ                "!="
LSS                "<"
GRT                ">"
LEQ                "<="
GEQ                ">="
AND                "&&"
OR                 "||"
-- Sonstige Token
COMMA              ","
SEMICOLON          ";"
LPAREN             "("
RPAREN             ")"
LBRACE             "{"
RBRACE             "}"
-- Termvariablen
CONST_INT          (Folge aus einer oder mehreren Ziffern)
CONST_FLOAT        (Eine Float-Zahl (siehe Pseudotokens) mit einem optionalen Exponententeil beschrieben durch ein "e" oder "E" und einem Integer mit optionalen Vorzeichen als Exponent)          ODER (Ein Integer gefolgt von einem "e" oder "E" und einem Integer mit optionalen Vorzeichen als Exponent)
CONST_BOOLEAN      ("true" oder "false")
CONST_STRING       (Beliebige Zeichenfolge zwischen Anführungszeichen)
ID                 (Folge aus mehreren Ziffern oder Buchstaben, beginnend immer mit einem Buchstaben)

-- "Pseudotoken" (nur zur Konstruktion anderer Token) --
DIGIT             [0-9]
INTEGER           [0-9]+
FLOAT             [0-9]+ "." [0-9]+ | "." [0-9]+
LETTER            [a-zA-Z]

-- Kommentare --
C-Kommentare      (Beginnend mit der Sequenz "/*" und endend mit der Sequenz "*/", Zeilenumbreüche im Kommentar sind zulässig)
C++-Kommentare    (Beginnend mit der Sequenz "//" und endend mit der Sequenz "\n", Zeilenumbreüche im Kommentar sind nicht zulässig)
```

Zusätzlich sind folgende Punkte zu beachten:

- Die Erkennung von Whitespaces (Leerzeichen, Tabulatoren, Zeilenenden) haben wir Ihnen bereits vorgegeben.
- C- (/* */) und C++ - (//) Kommentare sollen überlesen werden.
- Alle nicht in C1 erlaubten Zeichen (z.B. "%") sollen zur Rückgabe der `Error`-Variante führen.

