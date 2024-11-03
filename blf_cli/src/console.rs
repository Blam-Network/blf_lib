use std::time::SystemTime;
use inline_colorization::*;

pub struct console_task {
    messages: Vec<String>,
    warnings: Vec<String>,
    errors: Vec<String>,
    start_time: SystemTime,
}

#[macro_export]
macro_rules! debug_log {
    () => {
        std::print!("\n")
    };
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        {
            print!("##### DEBUG: ");
            print!($($arg)*);
            print!(" #####");
            println!();
        }
    }};
}

impl console_task {
    pub fn start(task: impl Into<String>) -> Self {
        print!("● {}... ", task.into());
        console_task {
            messages: vec![],
            warnings: vec![],
            errors: vec![],
            start_time: SystemTime::now()
        }
    }
    pub fn fail(&self) {
        println!("{color_red}failed{style_reset}.");
    }

    pub fn fail_with_error(&self, error: impl Into<String>) {
        println!("{color_red}failed{style_reset}.");
        Self::log_error(&error.into())
    }

    pub fn complete(&self){
        println!("{color_green}done ✓{style_reset}{} {}",
                 if self.errors.len() > 0 { format!(" ⛔  {} Errors", self.errors.len()) } else { String::new() },
                 if self.warnings.len() > 0 { format!(" ⚠ {} Warnings", self.warnings.len()) } else { String::new() }
        );
        for error in &self.errors {
            Self::log_error(error);
        }
        for warning in &self.warnings {
            Self::log_warning(warning);
        }
        for error in &self.messages {
            Self::log_message(error);
        }

        self.log_duration();
    }

    fn log_duration(&self) {
        let seconds = self.start_time.elapsed().unwrap().as_secs_f32();
        if seconds > 5f32 {
            println!("  ⏱ Task completed in {seconds:.2} seconds", );
        }
    }

    fn log_message(message: &String) {
        println!("  ⓘ {message}");
    }

    fn log_error(message: &String) {
        println!("  ⛔  {style_bold}{color_black}{bg_bright_red}{message}{style_reset}");
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    pub fn add_error(&mut self, message: String) {
        self.errors.push(message);
    }

    fn log_warning(message: &String) {
        println!("  ⚠ {style_bold}{color_black}{bg_bright_yellow}{message}{style_reset}");
    }
}