# Commit bodies

The title is a Conventional Commit; its format belongs to the `conventional-commits` skill. The body says what changed and why, in one to three sentences. Leave it out when the title says everything.

## What and why, not how

Before:

```text
fix: keep booked nights on reschedule

So basically I looked into this and it turned out that the problem was that when
an event is rescheduled we were deleting all the nights and recreating them, which
meant bookings got lost. I changed the code so that it now moves them instead.
Also cleaned up some stuff while I was at it.
```

After:

```text
fix: keep booked nights on reschedule

Rescheduling deleted and recreated every night, which dropped their bookings.
Nights now move with the event; nights outside the new dates are cancelled and
their guests notified.
```

The cleanup goes into its own commit.

## Skip the obvious

Before:

```text
ai: link the debug skill in AGENTS.md

This commit adds a link to the debug skill in the Skills section of AGENTS.md.
```

After:

```text
ai: link the debug skill in AGENTS.md
```

## Name the effect on the reader

Before:

```text
ci: update workflow

Updated the workflow file.
```

After:

```text
ci: fetch the full history in CI

moon ci compares the branch with main to find affected tasks, which fails on a
shallow clone.
```
