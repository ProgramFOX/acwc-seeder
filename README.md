# ACWC seeder

Finds seeding ratings for the Qualification part of the [Antichess World Championship 2019](https://acwc.chessvariants.training/).

## Setup and usage

The tool makes use of the [Lichess database PGN exports](https://database.lichess.org/).

You need the Antichess database file from September 2018 (included) up to August 2019 (included).

Place these files in their separate directory. This directory must have no less and no more files
than the aforementioned database export files!

To build from source and run:

```
cargo run --release -- path/to/database/exports/directory lichess-username
```
