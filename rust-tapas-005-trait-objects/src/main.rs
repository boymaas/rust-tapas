use std::any::Any;

// use as_any::{AsAny, Downcast};

struct Editor;

#[derive(Debug)]
struct FileExplorer;

struct StatusBar;

trait Plugin: Any {
    fn name(&self) -> String;
}

impl Plugin for FileExplorer {
    fn name(&self) -> String {
        "File Explorer".to_string()
    }
}
impl Plugin for StatusBar {
    fn name(&self) -> String {
        "Status Bar".to_string()
    }
}

fn main() {
    let plugins: Vec<Box<dyn Plugin>> = vec![Box::new(FileExplorer), Box::new(StatusBar)];

    // lets find the first status bar and print it
    // and use downcasting
    //
    for plugin in &plugins {
        // let any_type = plugin as &dyn Any;
        if let Some(status_bar) = plugin.as_ref().downcast_ref::<StatusBar>() {
            println!("Found status bar: {:?}", status_bar.name());
        }
    }
}
