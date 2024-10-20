use inline_colorization::*;

pub struct console_task {
    messages: Vec<String>,
    warnings: Vec<String>,
    errors: Vec<String>
}

impl console_task {
    pub fn start(task: String) -> Self {
        print!("● {}... ", task);
        console_task {
            messages: vec![],
            warnings: vec![],
            errors: vec![]
        }
    }
    pub fn fail(&self, error: String) {
        println!("{color_red}failed{style_reset}.");
        Self::log_error(&error)
    }

    pub fn complete(&self){
        println!("{color_green}done{style_reset}.{} {}",
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
    }

    fn log_message(message: &String) {
        println!("  ⓘ {message}");
    }

    fn log_error(message: &String) {
        println!("  ⛔ {color_bright_white}{bg_red}{message}{style_reset}");
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