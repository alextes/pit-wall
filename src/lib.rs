use std::time::{Duration, Instant};

/// The struct holding the state and functions related to our progress.
pub struct Progress {
    name: String,
    pub work_done: u64,
    started_at: Instant,
    work_total: u64,
}

impl Progress {
    /// Makes a progress object to track the work done.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice to identify the job we're tracking progress for.
    /// * `work_todo` - The units of work that need to be done to reach 100% progress.
    ///
    /// # Examples
    ///
    /// ```
    /// use pit_wall::Progress;
    /// let mut progress = Progress::new("my job", 100);
    /// ```
    pub fn new(name: &str, work_todo: u64) -> Self {
        Self {
            name: name.to_owned(),
            started_at: Instant::now(),
            work_done: 0,
            work_total: work_todo,
        }
    }

    /// Increment work done by one unit.
    ///
    /// ```
    /// use pit_wall::Progress;
    /// let mut progress = Progress::new("my job", 100);
    /// progress.inc_work_done();
    /// assert_eq!(progress.work_done, 1);
    /// ```
    pub fn inc_work_done(&mut self) {
        self.work_done = self.work_done + 1;
    }

    /// Increment work done by a given amonut.
    pub fn inc_work_done_by(&mut self, units: u64) {
        self.work_done = self.work_done + units;
    }

    /// Set work done.
    pub fn set_work_done(&mut self, units: u64) {
        self.work_done = units;
    }

    /// Get an estimate in seconds of the estimated seconds remaining.
    /// Uses basic linear interpolation to come up with an estimate.
    fn estimate_time_left(&self) -> Duration {
        let work_not_done = self.work_total - self.work_done;
        let not_done_to_done_ratio = work_not_done as f64 / self.work_done as f64;
        let seconds_since_start = Instant::now() - self.started_at;
        let eta_seconds = not_done_to_done_ratio * seconds_since_start.as_secs() as f64;

        Duration::from_secs(eta_seconds as u64)
    }

    /// Returns a formatted string giving a bunch of information on the current progress.
    /// You may want to log this periodically with whatever logging you have set up.
    pub fn get_progress_string(&self) -> String {
        let time_elapsed = format!("{:.0?}", Instant::now().duration_since(self.started_at));

        let eta = if self.work_done == self.work_total {
            "done!".to_string()
        } else {
            format!("{:.0?}", self.estimate_time_left())
        };

        format!(
            "{} {}/{} - {:.1}% started {} ago, eta: {}",
            self.name,
            self.work_done,
            self.work_total,
            self.work_done as f64 / self.work_total as f64 * 100f64,
            time_elapsed,
            eta
        )
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn test_inc_work_done() {
        let mut progress = Progress::new("test progress", 100);
        progress.inc_work_done();
        assert_eq!(progress.work_done, 1);
    }

    #[test]
    fn test_inc_work_done_by() {
        let mut progress = Progress::new("test progress", 100);
        progress.inc_work_done_by(10);
        assert_eq!(progress.work_done, 10);
    }

    #[test]
    fn test_set_work_done() {
        let mut progress = Progress::new("test progress", 100);
        progress.set_work_done(50);
        assert_eq!(progress.work_done, 50);
    }

    #[test]
    fn test_estimate_eta() {
        let mut progress = Progress::new("test progress", 100);
        progress.set_work_done(50);
        thread::sleep(Duration::from_secs(1));
        let eta = progress.estimate_time_left();
        assert_eq!(eta, Duration::from_secs(1));
    }

    #[test]
    fn test_get_progress_string() {
        let mut progress = Progress::new("test progress", 100);
        progress.set_work_done(50);

        // something like `test progress 50/100 - 50.0% started 41ns ago, eta: 0ns`
        // time elapsed will differ from test to test so we skip testing.
        let progress_string = progress.get_progress_string();

        assert!(progress_string.starts_with("test progress 50/100 - 50.0% started"));
        assert!(progress_string.ends_with("ago, eta: 0ns"));
    }

    #[test]
    fn test_work_complete_string() {
        let mut progress = Progress::new("test progress", 100);
        progress.set_work_done(100);

        // something like `test progress 50/100 - 50.0% started 41ns ago, eta: 0ns`
        // time elapsed will differ from test to test so we skip testing.
        let progress_string = progress.get_progress_string();

        assert!(progress_string.starts_with("test progress 100/100 - 100.0% started"));
        assert!(progress_string.ends_with("ago, eta: done!"));
    }
}
