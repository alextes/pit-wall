# Pit Wall - 🏎⏱
Measure the progress of execution and report on it. The intention is to help you report the progress of some execution in whatever form you wish.

Feedback and PRs very welcome.

## Usage
```rs
use pit_wall::Progress;

let mut progress = Progress::new("job name", 100);
progress.inc_work_done();
println!("{}", progress.get_progress_string()); // job name 2/100 - 2.0% started 2s ago, eta: 98s
```

## Why not use lib X instead?
All progress reporting libs I found either took control of the entire terminal output, or did not show an eta estimate.

## Wishlist
* Tests that don't depend on luck (currently some tests depend on duration of test execution).
* Pass a logging function plus a period to have the lib log for you.
* More flexibility in how progress is formatted as string.
* More flexibility in how progress is periodically logged.
