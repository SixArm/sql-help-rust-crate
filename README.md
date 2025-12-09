# SQL Help

Inspect your SQL code for improvements.

This is a work in progress. Constructive feedback welcome.

Inspired by:

- https://github.com/ankane/strong_migrations

- https://github.com/ayarotsky/diesel-guard

Goals that are different than diesel-guard:

- Aim to work with any SQL file from any tool, and don't expect Diesel.

- Aim to work with Postgres SQL current syntax, and don't use the Rust SQL AST crate which limits to SQL 2011 standard.

- Aim to preserve formatting including case and whitespace, and don't change results into uppercase or single spaces all on one line.
