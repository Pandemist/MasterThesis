# Übungsblatt 1

## Allgemeine Hinweise

Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, mit null Punkten bewertet werden! Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen. Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.

## Abgabemodus

Sie können in ihrer Lösung beliebige Hilfstypen und Funktionen selbst definieren. Die Einbindung von externen Crates und zusätzlichen Modulen ist jedoch nicht zulässig. Ebenso dürfen grundlegende Struktur des hier mitgelieferten Templates nicht verändert werden. Wichtig ist ebenfalls, dass definierte Namen, Signaturen von Methoden und absolute Pfade nicht verändert werden dürfen. Die Teile der Dateien, die von Ihnen bearbeitet werden sollen, sind grundsätzlich mit dem Schlagwort `TODO` gekennzeichnet.

Mögliche Aufrufe des Programms:
- Sie können die Implementation mit `cargo run -- <param>` aufrufen. Für den Platzhalter `<param>` können sie eigene Aufgaben eingeben.
- Mit dem Aufruf `cargo test` werden die Testfälle gestartet.
- Mit `cargo test -- --nocapture` werden auch Konsolenausgaben bei der Ausführung der Tests angezeigt.

## Enthaltenen Dateien

Müssen bearbeitet und abgegeben werden:
- `src/calculator.rs`
- `src/printer.rs`

## Aufgabe 1 (50 Punkte)

### Lernziel

Das erste Übungsblatt dient zur Vertiefung Ihrer praktischen Erfahrungen in Rust. Sie implementieren einen simplen Syntaxbaum und mehrere Visitor gemäß dem [Visitor-Pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html).

### Kurzbeschreibung

Sie erhalten einen Syntaxbaum, der beliebige Rechenaufgaben mit Ziffern und den Operatoren ['+', '-', '*', '/'] speichert. Implementieren Sie nun einen Pretty-Printer und einen Rechner für diesen Syntaxbaum.

### Aufgabenstellung

In der Datei `syntree` ist die Enumeration des Syntaxbaums vorgegeben. Ebenfalls sind dort Funktionen vorgegeben, die Knoten/Bäume aus eingegebenem Text generieren können, wenn dieser dem Eingabeformat entspricht.

#### a) Implementierung des Pretty-Printers (20 Punkte)

Implementieren Sie einen Pretty-Printer für beliebige Syntaxbäume. Füllen Sie dazu die `visit_*` Funktionen in der Datei `printer.rs` gemäß dem [Visitor-Pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html) aus. Der Printer soll den Inhalt des Syntaxbaums hierarchisch geklammert ausgeben. Zuweisungen von Variablen sollen nicht ausgegeben werden.

#### b) Implementierung des Rechners (30 Punkte)

Implementieren Sie einen Rechner für beliebige Syntaxbäume. Füllen Sie dazu die `visit_*` Funktionen in der Datei `calculator.rs` gemäß dem [Visitor-Pattern](https://rust-unofficial.github.io/patterns/patterns/behavioural/visitor.html) aus. Der Rechner soll den Syntaxbaum gemäß der Klammerung des Printers (nach hierarchischer Klammerung) ausrechnen. Dabei sollen auch Variablen verwendet werden können. Als Variablen sind die Characters `[a-z]` zulässig. Variablen sollen von einem `Set`-Knoten gesetzt werden können. Mit einem `Var`-Knoten soll auf den jeweiligen Wert einer Variable zugegriffen werden. Die Werte der Variablen sollen im Calculator gespeichert werden. Sollte eine Teilaufgabe zu undefiniertem Verhalten führen, so soll das Programm sich mit einem `panic!` beenden. Überläufe eines Wertes sollen erkannt werden, und der Wert auf den maximalen/minimalen Wert des Zahlenbereichs von `i64` begrenzt werden.

