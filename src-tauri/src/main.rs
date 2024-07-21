use tauri::{generate_context, AppHandle, Emitter, Manager, State};

use config::{EmittedMarkdown, CONTENT_EVENT};
use futures::{
    channel::mpsc::{channel, Receiver},
    SinkExt, StreamExt,
};
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    io::{stdout, Write},
    path::Path,
    sync::Mutex,
};
use tauri_plugin_cli::CliExt;

const SLIDES_SPLITTER: &str = "---";

const HELP_MESSAGE: &[u8] = r#"
    Welcom to iquity 
        the markdown compiler

    you called iquity without a markdown path
    
        you should call the program with the path to
    the target md file then the program will hot reload
    the content of the file every time you change 
    something in it. 


    EXAMPLE

    iquity ./README.md


    PREVIEW WINDOW KEYS    

    p => print to pdf

    j => next theme

    k => previous theme

    L => next slide

    H => previous slide

    = or + => increase font size    

    - or _ => decrease font size    

    ? or / => show this help message    

    esc => to hide this message    
"#
.as_bytes();

struct Content {
    slides: Mutex<Vec<String>>,
    index: Mutex<usize>,
}

impl Default for Content {
    fn default() -> Self {
        Self {
            slides: Mutex::new(Vec::new()),
            index: Mutex::new(0),
        }
    }
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .manage(Content::default())
        .invoke_handler(tauri::generate_handler![md_init, next_slide, prev_slide,])
        .setup(move |app| {
            let matches = app.cli().matches().unwrap();
            let Some(path) = matches
                .args
                .get("path")
                .and_then(|x| x.value.as_str().map(|x| x.to_string()))
            else {
                stdout().write_all(HELP_MESSAGE).unwrap();
                std::process::exit(0x0100);
            };
            app.manage(path.clone());

            let handle = app.app_handle().to_owned();
            tokio::task::spawn(async move {
                watch(handle, path).await.unwrap();
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

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

async fn watch<P: AsRef<Path>>(app: AppHandle, path: P) -> Result<(), Box<dyn std::error::Error>> {
    let (mut watcher, mut rx) = watcher()?;
    watcher.watch(path.as_ref(), RecursiveMode::NonRecursive)?;

    let content = app.state::<Content>();

    loop {
        if rx.next().await.is_none() {
            continue;
        }
        let slides = read_file(&path).await?;
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

#[tauri::command]
async fn md_init(
    app: AppHandle,
    content: State<'_, Content>,
    path: State<'_, String>,
) -> Result<(), String> {
    let slides = read_file(&path.inner()).await.map_err(|x| x.to_string())?;
    let mut content_slides = content.slides.lock().unwrap();
    emit_content(&app, 0, slides.len(), &slides[0]);
    *content_slides = slides;
    Ok(())
}

async fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let text = tokio::fs::read_to_string(path).await?;
    let slides = text.split(SLIDES_SPLITTER).map(|x| x.to_string()).collect();
    Ok(slides)
}

#[tauri::command]
fn next_slide(app: AppHandle, content: State<'_, Content>) {
    let slides = content.slides.lock().unwrap();
    let mut index = content.index.lock().unwrap();
    let slide = if *index < slides.len() - 1 {
        *index += 1;
        slides.get(*index).unwrap()
    } else {
        slides.last().unwrap()
    };
    emit_content(&app, *index, slides.len(), slide);
}

#[tauri::command]
fn prev_slide(app: AppHandle, content: State<'_, Content>) {
    let slides = content.slides.lock().unwrap();
    let mut index = content.index.lock().unwrap();
    *index = index.checked_sub(1).unwrap_or(0);
    let slide = slides.get(*index).unwrap();
    emit_content(&app, *index, slides.len(), slide);
}

fn emit_content(app: &AppHandle, index: usize, len: usize, slide: &String) {
    let output = EmittedMarkdown::new(index + 1, len, slide);
    app.emit(CONTENT_EVENT, output).unwrap();
}
