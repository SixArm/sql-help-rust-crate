# Create index concurrently

Why use `CREATE INDEX CONCURRENTLY`? The SQL `CONCURRENTLY` keyword allows index creation without locking out writes to the table.

<https://www.postgresql.org/docs/current/sql-createindex.html#SQL-CREATEINDEX-CONCURRENTLY>

## Without CONCURRENTLY (default behavior)

```sql
CREATE INDEX my_index ON my_table (my_column);
```

What happens:

- 🔒 Acquires SHARE lock on the table
- ✅ Allows reads (SELECT)
- ❌ Blocks writes (INSERT, UPDATE, DELETE)
- ⏱️ Lock held for entire index build (minutes to hours on large tables)
- 💥 Production killer - can cause downtime or request queuing

## With CONCURRENTLY

```sql
CREATE INDEX CONCURRENTLY my_index ON my_table (my_column);
```

What happens:

- 🔓 Uses weaker locks that don't block writes
- ✅ Allows reads (SELECT)
- ✅ Allows writes (INSERT, UPDATE, DELETE)
- ⏱️ Takes longer to complete (more overhead)
- ✨ Zero downtime - production keeps running

## Real-world scenario

Suppose you have a production table with 100M rows.

Running CREATE INDEX might take 30 minutes,
and during those 30 minutes all writes are blocked!

Running CREATE INDEX might take 60 minutes,
and during those 60 minutes all writes work as usual.

## Trade-offs

| Aspect            | Regular               | CONCURRENTLY             |
|-------------------|-----------------------|--------------------------|
| Build time        | Faster                | ~2x slower               |
| Locks             | Blocks writes         | Doesn't block            |
| Disk space        | Less                  | More (temporary)         |
| Failure handling  | Rolls back cleanly    | Leaves invalid index     |
| Transactions      | Can be in transaction | Cannot be in transaction |
| Production safety | ⚠️ Dangerous          | ✅ Safe                   |

## Important gotchas

### Cannot run in transaction block

This fails:

```sql
BEGIN;
CREATE INDEX CONCURRENTLY my_index ON my_table (my_column);
COMMIT;
```

Must run standalone:

```sql
CREATE INDEX CONCURRENTLY my_index ON my_table (my_column);
```

### If it fails, it leaves invalid index

Check for invalid indexes:

```sql
SELECT indexrelid::regclass, indisvalid
FROM pg_index
WHERE NOT indisvalid;
```

Clean up:

```sql
DROP INDEX CONCURRENTLY my_index;
```

### Requires more resources

Requires extra disk space during build.

Requires more CPU because it validates data while writes continue.

Takes 2-3x longer than regular index.

## When to use each

Use regular `CREATE INDEX`:

- ✅ Development/testing environments
- ✅ New tables (no existing data)
- ✅ Maintenance windows with downtime
- ✅ Small tables (< 1M rows)

Use `CREATE INDEX CONCURRENTLY`:

- ✅ Production databases
- ✅ Large tables with active traffic
- ✅ Zero-downtime deployments
- ✅ 24/7 services

## Best practice pattern

Create index without blocking:

```sql
CREATE INDEX CONCURRENTLY IF NOT EXISTS my_index ON my_table (my_column);
```

Verify success:

```sql
SELECT schemaname, tablename, indexname, indexdef
FROM pg_indexes
WHERE indexname = 'my_index';
```

Verify validity:

```sql
SELECT indexrelid::regclass, indisvalid
FROM pg_index
WHERE indexrelid = 'my_index'::regclass;
```

## PostgreSQL internals

`CONCURRENTLY` performs a multi-phase build:

1. Phase 1: Scan table, build initial index (allows writes)
2. Phase 2: Wait for existing transactions to complete
3. Phase 3: Scan again for changes made during phase 1
4. Phase 4: Mark index as valid

This is why it's slower but doesn't block.

## Other databases

- MySQL/MariaDB: `ALGORITHM=INPLACE, LOCK=NONE`
- SQL Server: Online index operations (`ONLINE=ON`)
- Oracle: `CREATE INDEX ... ONLINE`

## Conclusion

In production, always use `CONCURRENTLY` unless you have a specific reason not
to. The performance hit is worth avoiding potential downtime.
