extern crate notify;

use notify::{Watcher, RecursiveMode, RawEvent, raw_watcher};
use std::sync::mpsc::channel;
use std::path::Path;
use std::fs;

fn move_file(from: &Path, to: &str) -> std::io::Result<()> {
    fs::rename(Path::new(&from), Path::new(&to))?;
    Ok(())
}

fn main() {
    let (tx, rx) = channel();
    let mut watcher = raw_watcher(tx).unwrap();
    
    watcher.watch("/Users/darioribeiro/Downloads", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
           Ok(RawEvent{path: Some(path), op: _, cookie:_}) => {
            
            let extension = Path::new(&path)
                .extension();

            let filename = Path::new(&path)
                .file_name().unwrap().to_str().unwrap();


            match extension {
                txt => {
                    let to = format!("/Users/darioribeiro/Desktop/{}", filename);
                    move_file(&path, &to);
                    println!("hahahah: {:?}", path);
                }
            };
            
           },
           Ok(event) => println!("broken event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}