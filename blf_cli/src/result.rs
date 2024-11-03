#[macro_export]
macro_rules! やった {
    () => {
        Ok(())
    };
    ($task:expr) => {
        return {
            $task.complete();
            Ok(())
        }
    }
}