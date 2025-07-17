use std::cmp;

use console::Color;
use indicatif::ProgressBar;
use infra_utils::github::GitHub;

use crate::reporting::Reporter;
use crate::results::ShardResults;

const MAX_PRINTED_FAILURES: u64 = 1000;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum TestOutcome {
    Passed,
    Failed,
    Unresolved,
    Incompatible,
}

pub struct Events {
    reporter: Reporter,

    all_archives: ProgressBar,
    current_archive: ProgressBar,

    source_files: ProgressBar,

    passed: ProgressBar,
    failed: ProgressBar,
    unresolved: ProgressBar,
    incompatible: ProgressBar,

    version8: ProgressBar,
    solar_passed: ProgressBar,
    solar_failed: ProgressBar,
}

impl Events {
    pub fn new(archives_count: usize, files_count: usize) -> Self {
        let mut reporter = Reporter::new();

        reporter.add_blank();

        let all_archives = reporter.add_progress("All Archives", archives_count);
        let current_archive = reporter.add_progress("Current Archives", 0);

        reporter.add_blank();

        let source_files = reporter.add_counter("📄 Source Files", Color::White, files_count);

        reporter.add_blank();
        reporter.add_label("Contract Stats:");

        let passed = reporter.add_counter("✅ Passed", Color::Green, 0);
        let failed = reporter.add_counter("❌ Failed", Color::Red, 0);
        let unresolved = reporter.add_counter("❔ Unresolved", Color::White, 0);
        let incompatible = reporter.add_counter("❕ Incompatible", Color::White, 0);
        let version8 = reporter.add_counter("#0.8 Version 0.8", Color::White, 0);
        let solar_passed = reporter.add_counter("☀︎✅ Solar passed", Color::Color256(10), 0);
        let solar_failed = reporter.add_counter("☀︎❌ Solar failed", Color::Color256(13), 0);

        reporter.add_blank();

        Self {
            reporter,

            all_archives,
            current_archive,

            source_files,

            passed,
            failed,
            unresolved,
            incompatible,

            version8,
            solar_passed,
            solar_failed,
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn failure_count(&self) -> usize {
        self.failed.position() as usize
    }

    pub fn start_archive(&mut self, contract_count: usize) {
        self.current_archive.reset();
        self.current_archive.set_length(contract_count as u64);

        self.reporter.show();
    }

    pub fn inc_files_count(&self, additional_files: usize) {
        self.source_files.inc_length(additional_files as u64);
    }

    pub fn inc_files_processed(&self, files_processed: usize) {
        self.source_files.inc(files_processed as u64);
    }

    pub fn finish_archive(&mut self) {
        self.all_archives.inc(1);

        self.reporter.hide();

        GitHub::collapse_group("Statistics:", || {
            self.reporter.print_full_report();
        });
    }

    pub fn test(&self, outcome: TestOutcome) {
        self.current_archive.inc(1);

        self.passed.inc_length(1);
        self.failed.inc_length(1);
        self.unresolved.inc_length(1);
        self.incompatible.inc_length(1);

        match outcome {
            TestOutcome::Passed => self.passed.inc(1),
            TestOutcome::Failed => self.failed.inc(1),
            TestOutcome::Unresolved => self.unresolved.inc(1),
            TestOutcome::Incompatible => self.incompatible.inc(1),
        }
    }

    pub fn solar_test(&self, outcome: TestOutcome) {
        self.version8.inc_length(1);
        self.version8.inc(1);
        self.solar_passed.inc_length(1);
        self.solar_failed.inc_length(1);

        match outcome {
            TestOutcome::Passed => self.solar_passed.inc(1),
            TestOutcome::Failed => self.solar_failed.inc(1),
            _ => panic!("Unexpected outcome for solar"),
        }
    }

    fn test_error(&self, message: impl AsRef<str>) {
        match self.failed.position().cmp(&MAX_PRINTED_FAILURES) {
            cmp::Ordering::Less => {
                self.reporter.println(message);
            }
            cmp::Ordering::Equal => {
                self.reporter.println(format!(
                    "More than {MAX_PRINTED_FAILURES} failures shown. Additional failures will be silent."
                ));
            }
            cmp::Ordering::Greater => {
                // Don't print any more messages...
            }
        }
    }

    pub fn parse_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn version_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn bindings_error(&self, message: impl AsRef<str>) {
        self.test_error(message);
    }

    pub fn trace(&self, message: impl AsRef<str>) {
        self.reporter.println(message);
    }

    pub fn to_results(&self) -> ShardResults {
        ShardResults {
            source_files: self.source_files.position(),
            passed: self.passed.position(),
            failed: self.failed.position(),
            unresolved: self.unresolved.position(),
            incompatible: self.incompatible.position(),
            elapsed: self.all_archives.elapsed(),
        }
    }
}
