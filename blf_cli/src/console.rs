use inline_colorization::*;

pub struct console_task {
    warnings: Vec<String>
}

impl console_task {
    pub fn start(task: String) -> Self {
        print!("● {}... ", task);
        console_task {
            warnings: vec![]
        }
    }
    pub fn fail(&self, error: String) {
        println!("{color_red}failed{style_reset}.");
        Self::log_error(error)
    }

    pub fn complete(&self){
        println!("{color_green}done{style_reset}.{}", if self.warnings.len() > 0 { format!(" ⚠ {} Warnings", self.warnings.len()) } else { String::new() });
        for warning in &self.warnings {
            Self::log_warning(warning);
        }
    }

    fn log_error(message: String) {
        println!("❗{color_bright_white}{bg_red}{message} {style_reset}");
    }

    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }

    fn log_warning(message: &String) {
        println!("  ⚠ {style_bold}{color_black}{bg_bright_yellow} {message} {style_reset}");
    }
}