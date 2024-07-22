use super::{Content, SLIDES_SPLITTER};

use config::{EmittedMarkdown, CONTENT_EVENT};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};

use tauri::{AppHandle, Emitter, Manager};

use std::path::Path;

fn watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut tx, rx) = channel(1);

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            })
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}

pub async fn watch<P: AsRef<Path>>(
    app: AppHandle,
    path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut watcher, mut rx) = watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    let content = app.state::<Content>();

    loop {
        if rx.next().await.is_none() {
            continue;
        }
        let slides = read_markdown(&path).await?;
        let mut content_slides = content.slides.lock().unwrap();
        *content_slides = slides;
        let mut index = content.index.lock().unwrap();
        if *index > content_slides.len() - 1 {
            *index = content_slides.len() - 1;
        };
        emit_content(
            &app,
            *index,
            content_slides.len(),
            content_slides.get(*index).unwrap_or(&String::new()),
        );
    }
}

pub async fn read_markdown<P: AsRef<Path>>(
    path: P,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let text = tokio::fs::read_to_string(path).await?;
    let slides = text.split(SLIDES_SPLITTER).map(|x| x.to_string()).collect();
    Ok(slides)
}

pub fn emit_content(app: &AppHandle, index: usize, len: usize, slide: &String) {
    let output = EmittedMarkdown::new(index + 1, len, slide);
    app.emit(CONTENT_EVENT, output).unwrap();
}
