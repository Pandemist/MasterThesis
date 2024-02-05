# Übungsblatt 4
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

- `src/semantic_checker.rs`

## Aufgabe 1 (50 Punkte)

### Lernziel
Das Lernziel dieser Aufgabe ist es, die Auseinandersetzung mit der semantischen Überprüfung. Dabei werden Sie sich mit Sichtbarkeiten und Typenüberprüfung beschäftigen.

### Kurzbeschreibung
Implementieren Sie die semantische Analyse mithilfe der mitgelieferten Datei `symbol_table.rs` und den bereits im `semantic_checker` enthaltenen Funktionen. Als Eingabe wird ein Syntaxbaum übergeben.

### Aufgabenstellung
Implementieren Sie die semantische Analyse in der dazu vorbereiteten Datei `semantic_checker.rs`. Dazu steht Ihnen neben den dort bereits implementierten Funktionen auch die mitgelieferte Symboltabelle `symbol_table.rs` zur Verfügung.
Neben der semantischen Analyse soll auch der Syntaxbaum umgearbeitet werden. Dazu wird eine neue Zielstruktur mitgeliefert `refactored_syntax_tree.rs`. Dabei müssen Variablennamen durch ihre Position im Stack ersetzt und Funktionsnamen aufgelöst und durch direkte Referenzen auf den Funktionsknoten ersetzt werden.

Die zu prüfende Programmsemantik finden Sie hier. Diese muss vollständig umgesetzt werden.

### Semantik

#### Sichtbarkeiten
Ein Sichtbarkeitsbereich ist ein Abschnitt des Quellcodes, der zu folgenden Knoten des Syntaxbaums reduziert wird:

- NdBlock
- NdFunction
- NdFor
- NdDoWhile
- NdWhile

Die Grammatik erlaubt eine Schichtung von Sichtbarkeitsbereichen. Die äußerste Schicht wird als die globale Schicht bezeichnet. Dort deklarierte Variablen gelten als globale Variablen. Jede weitere Schicht wird auf einen Stapel gelegt. Bei jeder Auflösung eines Bezeichners werden die Sichtbarkeitsbereiche von innen nach außen durchsucht, und der erste Treffer wird gewählt.

#### Funktionen
- Es muss eine parameterlose `main()`-Funktion vom Typ `void` geben.
- Namen von Funktionen müssen einmalig sein.
- Eine Funktion muss vor ihrem Aufruf deklariert werden.
- Bei einem Funktionsaufruf muss die Übereinstimmung der Parameter und der Argumente geprüft werden (d. h. gleiche Menge und paarweise gleiche Typen).
- Der Rückgabewert einer Funktion muss den gleichen Typ haben wie die Funktion, in der sich das Return Statement befindet.

#### Typen
Eine implizite Konvertierung von einem in einen anderen Typ ist nur bedingt zulässig. Ebenfalls gilt:

- Variablen dürfen nicht vom Typ `void` sein.
- Ausdrücke des Typs `void` können nicht von `printf()` ausgegeben werden.
- Die Konvertierung von `integer` zu `float` ist zulässig. Nutzen Sie dazu die Enumerationsvarianten `IntToFloat` um eine solche Konvertierung in der Struktur dar zu stellen.

Alle hier nicht beachteten Fälle werden entsprechend des [C-Standards](https://web.archive.org/web/20181230041359if_/http://www.open-std.org/jtc1/sc22/wg14/www/abq/c17_updated_proposed_fdis.pdf) behandelt.

