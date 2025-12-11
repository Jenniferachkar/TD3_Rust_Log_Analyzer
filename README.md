# TD3 Rust Log Analyzer (CLI)

Un analyseur de logs en ligne de commande écrit en Rust.

## Format des logs attendu

Chaque ligne doit respecter le motif suivant :

```
YYYY-MM-DD HH:MM:SS [NIVEAU] Message libre
```

Exemple :

```
2024-01-31 10:31:15 [ERROR] Failed to connect to API
```

Le parseur utilise l'expression régulière `^(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}) \[(\w+)\] (.+)$` pour extraire `timestamp`, `level` et `message`. Toute ligne qui ne matche pas est ignorée.

## Algorithme utilisé

- Charge le fichier en mémoire et parcourt chaque ligne via un `BufReader`.
- Parse chaque ligne avec la regex ci-dessus pour produire des `LogEntry { timestamp, level, message }`.
- Si `--errors-only` est activé, ne conserve que les entrées dont `level == "ERROR"`.
- Exporte le résultat selon le format choisi :
  - `text` (défaut) : tableau via `prettytable`.
  - `json` : sérialisation `serde_json` indentée.
  - `csv` : en-tête `timestamp,level,message` puis valeurs.
- Le mode `--verbose` affiche le chemin lu et le nombre d'entrées parsées.

## Limitations connues

- Format strict : pas de prise en charge des logs multilignes ou des formats horodatés alternatifs.
- Les lignes invalides sont simplement ignorées (aucun rapport d'erreur ligne par ligne).
- Analyse en mémoire : les très gros fichiers ne sont pas streamés ni découpés par blocs.
- Pas de regroupement par niveau/sous-système ni de statistiques (comptes, histogrammes, etc.).
- Pas de support temps réel ou de suivi de fichiers en cours d'écriture.

## Benchmarks

Pas encore de benchmarks intégrés ou publiés. Un scénario simple peut être mesuré avec `hyperfine "cargo run -- sample.log"`. Ajouter des échantillons volumineux (1M+ lignes) et mesurer la consommation mémoire serait une bonne prochaine étape.

## Aperçu (capture)

![Sortie en mode texte](docs/screenshot-output.png)

## Exécution

```bash
cargo run -- sample.log
```
Mode verbeux :
```bash
cargo run -- sample.log --verbose
```

Erreurs uniquement :
```bash
cargo run -- sample.log --errors-only
```

JSON :
```bash
cargo run -- sample.log --format json
```

CSV :
```bash
cargo run -- sample.log --format csv
```
