use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

/// Create a spinner for indeterminate progress
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );

    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));

    pb
}

/// Create a progress bar for determinate progress
pub fn create_progress_bar(total: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(total);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({percent}%)")
            .unwrap()
            .progress_chars("#>-"),
    );

    pb.set_message(message.to_string());

    pb
}

/// Create a progress bar for file downloads
pub fn create_download_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);

    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    pb
}

/// Create a multi-step progress indicator
pub struct MultiStepProgress {
    steps: Vec<String>,
    current: usize,
    pb: ProgressBar,
}

impl MultiStepProgress {
    pub fn new(steps: Vec<String>) -> Self {
        let total = steps.len() as u64;
        let pb = ProgressBar::new(total);

        pb.set_style(
            ProgressStyle::default_bar()
                .template(
                    "{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}",
                )
                .unwrap()
                .progress_chars("=>-"),
        );

        Self {
            steps,
            current: 0,
            pb,
        }
    }

    pub fn start_step(&mut self) {
        if self.current < self.steps.len() {
            let msg = format!("Step {}: {}", self.current + 1, self.steps[self.current]);
            self.pb.set_message(msg);
            self.pb.set_position(self.current as u64);
        }
    }

    pub fn complete_step(&mut self) {
        self.current += 1;
        if self.current < self.steps.len() {
            self.start_step();
        } else {
            self.pb.finish_with_message("All steps completed!".to_string());
        }
    }

    pub fn fail_step(&self, error: &str) {
        self.pb.abandon_with_message(format!("Failed: {}", error));
    }
}
