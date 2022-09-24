# Pit Wall - 🏎⏱

Measure the progress of execution and report on it. The intention is to help you report the progress of some execution in whatever form you wish. Not a profiling library. You define, and report the work done yourself.

Feedback and PRs very welcome.

## Why not use lib X instead?

All progress reporting libs I found either took control of the entire terminal output, or did not show an eta estimate.

## Wishlist

- Pass a logging function plus a period to have the lib log for you.
- More flexibility in how progress is formatted as string.
- More flexibility in how progress is periodically logged.
- Test time_elapsed output more rigorously.
- Start at time for long-running processes that are resumed.
