# Übungsblatt 5

## Allgemeine Hinweise
Für diese und alle folgenden Praktikumsaufgaben gilt, dass Einsendungen, die in der jeweils mitgegebenen Testumgebung nicht laufen, mit null Punkten bewertet werden! Das beinhaltet insbesondere alle Programme, die sich nicht fehlerfrei kompilieren lassen. Da Cargo für die Ausführung verantwortlich ist, sollte das Projekt bei Ihnen am Ende mit `cargo test` ohne Fehler und Warnungen durchlaufen.

## Abgabemodus
Sie können in ihrer Lösung beliebige Hilfstypen und Funktionen selbst definieren. Die Einbindung von externen Crates und zusätzlichen Modulen ist jedoch nicht zulässig. Ebenso darf die grundlegende Struktur des hier mitgelieferten Templates nicht verändert werden. Wichtig ist ebenfalls, dass definierte Namen, Signaturen von Methoden und absolute Pfade nicht verändert werden dürfen. Die Teile der Dateien, die von Ihnen bearbeitet werden sollen, sind grundsätzlich mit dem Schlagwort `TODO` gekennzeichnet.

Mögliche Aufrufe des Programms:

- Sie können die Implementation mit `cargo run -- <param>` aufrufen. Für den Platzhalter `<param>` können Sie eigene Dateipfade angeben.
- Mit dem Aufruf `cargo test` werden die Testfälle gestartet.
- Mit `cargo test -- --nocapture` werden auch Konsolenausgaben bei der Ausführung der Tests angezeigt.

## Enthaltenen Dateien
Muss bearbeitet und abgegeben werden:

- `src/interpreter.rs`

## Aufgabe 1 (100 Punkte)

### Lernziel
Ziel dieser Aufgabe ist die Implementierung eines Interpreters. Dabei beschäftigen Sie sich mit dem C-Call-Stack.

### Aufgabenstellung
Vervollständigen Sie den mitgelieferten Interpreter in der Datei `interpreter.rs`. Sie dürfen in dieser Aufgabe beliebige Änderungen an der Datei `interpreter.rs` vornehmen, soweit Sie es für nötig erachten. Jedoch dürfen die Funktionssignaturen nicht verändert werden.

Wir wünschen Ihnen viel Erfolg beim Lösen dieser Aufgabe und hoffen, dass Sie genau so viel Spaß bei der Umsetzung dieses Projekts hatten wie wir bei der Konzeption!

