# Übungsblatt 3
## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, mit null Punkten bewertet werden! Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen. Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.

## Abgabemodus
Sie können in ihrer Lösung beliebige Hilfstypen und Funktionen selbst definieren. Die Einbindung von externen Crates und zusätzlichen Modulen ist jedoch nicht zulässig. Ebenso dürfen grundlegende Struktur des hier mitgelieferten Templates nicht verändert werden. Wichtig ist ebenfalls, dass definierte Namen, Signaturen von Methoden und absolute Pfade nicht verändert werden dürfen. Die Teile der Dateien, die von Ihnen bearbeitet werden sollen, sind grundsätzlich mit dem Schlagwort `TODO` gekennzeichnet.

Mögliche Aufrufe des Programms:
- Sie können die Implementation mit `cargo run -- <param>` aufrufen. Für den Platzhalter `<param>` können sie eigene Dateipfade angeben.
- Mit dem Aufruf `cargo test` werden die Testfälle gestartet.
- Mit `cargo test -- --nocapture` werden auch Konsolenausgaben bei der Ausführung der Tests angezeigt.

## Enthaltenen Dateien
Muss bearbeitet und abgegeben werden:
- `src/c1_pars.lalrpop`

## Aufgabe 1 (50 Punkte)
### Lernziel
Das Ziel dieser Übungsaufgabe ist es, dass Sie sich mit dem Konzept eines Parsers auseinandersetzen. Der Fokus wird dabei auch auf der Struktur von Grammatiken liegen. Besonders wird die Eigenschaft der Eindeutigkeit einer Grammatik eine Rolle spielen.

### Kurzbeschreibung
Implementieren Sie mithilfe des Parsergenerators [lalrpop](https://crates.io/crates/lalrpop) einen Parser für die Sprache C1. Der Parsergenerator erwartet dabei einen Tokenstrom des Lexers.

### Aufgabenstellung
Ihre Aufgabe besteht darin, einen Parser für die Sprache C1 zu erstellen. Dieser Parser soll dabei einen Syntaxbaum aus den Tokens aufbauen. Legen Sie dazu die Regeln der Grammatik für die Sprache C1 in der mitgelieferten Datei `c1_pars.lalrpop` an. Der Parser soll nach vollständigem Konsumieren der Tokens einen Syntaxbaum zurückgeben. Die Enumerationen und Strukturen, aus der Sie den Syntaxbaum aufbauen sollen, sind in der Datei `syntax_tree.rs` vorgegeben. Diese darf nicht verändert oder ergänzt werden. Zur Orientierung haben wir Ihnen die Regelnköpfe für die Startregel (zur Zeit auskommentiert, damit das Programm sich von beginn an bauen lässt) und die Terminale vorgegeben.

Von lalrpop können nur **eindeutige** Grammatiken akzeptiert werden. Es ist Ihre Aufgabe, eventuelle auftretende Mehrdeutigkeiten in der mitgelieferten Grammatik selbst aufzulösen. Grundsätzlich ist es zulässig, die Form der mitgelieferten Grammatik zu verändern. Dabei darf sich die Menge der akzeptierten Eingaben der Grammatik allerdings <ins>nicht</ins> verändern.

Die ursprüngliche Grammatik von C1 finden Sie nachfolgend dargestellt.
```c
program             ::= ( declassignment ";" | functiondefinition )*

functiondefinition  ::= type id "(" ( parameterlist )? ")" "{" statementlist "}"
parameterlist       ::= type id ( "," type id )*
functioncall        ::= id "(" ( assignment ( "," assignment )* )? ")"

statementlist       ::= ( block )*
block               ::= "{" statementlist "}"
                      | statement

statement           ::= ifstatement
                      | forstatement
                      | whilestatement
                      | simple_statement
simple_statement	::= returnstatement ";"
                      | dowhilestatement ";"
                      | printf ";"
                      | declassignment ";"
                      | statassignment ";"
                      | functioncall ";"
                      | ";"
                      
statblock           ::= "{" statementlist "}"
                      | statement

ifstatement         ::= <KW_IF> "(" assignment ")" statblock ( <KW_ELSE> statblock )?
forstatement        ::= <KW_FOR> "(" ( statassignment | declassignment ) ";" expr ";" statassignment ")" statblock
dowhilestatement    ::= <KW_DO> statblock <KW_WHILE> "(" assignment ")"
whilestatement      ::= <KW_WHILE> "(" assignment ")" statblock
returnstatement     ::= <KW_RETURN> ( assignment )?
printf              ::= <KW_PRINTF> "(" (assignment | CONST_STRING) ")"
declassignment      ::= type id ( "=" assignment )?
type                ::= <KW_BOOLEAN>
                      | <KW_FLOAT>
                      | <KW_INT>
                      | <KW_VOID>

statassignment      ::= id "=" assignment
assignment          ::= id "=" assignment
                      | expr
expr                ::= simpexpr ( "==" simpexpr | "!=" simpexpr | "<=" simpexpr | ">=" simpexpr | "<" simpexpr | ">" simpexpr )?
simpexpr            ::= ( "-" term | term ) ( "+" term | "-" term | "||" term )*
term                ::= factor ( "*" factor | "/" factor | "&&" factor )*
factor              ::= <CONST_INT>
                      | <CONST_FLOAT>
                      | <CONST_BOOLEAN>
                      | functioncall
                      | id
                      | "(" assignment ")"
id                  ::= <ID>
```

