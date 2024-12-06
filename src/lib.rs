use anyhow::anyhow;
use std::panic;

mod day01;

// This can be implemented with panics internally, no need for error handling.
trait RawSolution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

// Catch any panics, convert them to a generic result.
fn catch_all<T>(f: impl FnOnce() -> T + panic::UnwindSafe) -> anyhow::Result<T> {
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let out = panic::catch_unwind(f).map_err(|e| anyhow!("panic: {:?}", e.downcast_ref::<&str>()));
    panic::set_hook(previous_hook);
    out
}

/// Represents a solution to a particular day of advent of code.
pub struct Solution {
    inner: Box<dyn RawSolution + panic::RefUnwindSafe>,
}

impl<T: RawSolution + panic::RefUnwindSafe + 'static> From<T> for Solution {
    fn from(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

impl Solution {
    pub fn part1(&self, input: &str) -> anyhow::Result<String> {
        catch_all(|| self.inner.part1(input))
    }

    pub fn part2(&self, input: &str) -> anyhow::Result<String> {
        catch_all(|| self.inner.part2(input))
    }

    /// Get the solution for a particular day.
    ///
    /// That day may not have been implemented yet, hence `Option`.
    pub fn at(day: u32) -> Option<Solution> {
        match day {
            1 => Some(day01::Solution.into()),
            _ => None,
        }
    }
}
