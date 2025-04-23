# DJibon Frontend (Dioxus)

This directory contains the Dioxus-based frontend implementation for the DJibon Personal Assistance and Communication Tool (PACT).

## Why Dioxus over Flutter?

### Rapid Prototyping & Development

- **Faster Development Cycle**: Dioxus offers a significantly faster development cycle compared to Flutter, with hot-reloading that feels almost magical. Changes to formatted strings, component properties, and nested RSX blocks are instantly reflected without full rebuilds.
  
- **Simpler Testing**: Testing Dioxus applications is more straightforward, especially for web interfaces, as it doesn't require complex emulator setups for basic functionality testing.

- **Rust-Native**: Being a pure Rust framework, Dioxus eliminates the need to learn Dart, reducing the cognitive load for developers already familiar with Rust.

### Cross-Platform Capabilities

- **Unified Codebase**: Dioxus 0.6+ supports web, desktop, and mobile platforms from a single codebase with minimal platform-specific code.

- **Web-First Approach**: Dioxus was designed with web as a first-class citizen, making it ideal for applications that prioritize web interfaces while still supporting native platforms.

- **WASM Optimization**: Dioxus leverages WebAssembly for web deployment, offering better performance than Flutter's HTML/CSS/JS compilation approach.

### Integration with SQLite

Dioxus works seamlessly with SQLite through several Rust crates:

- **rusqlite**: A robust SQLite binding for Rust that provides low-level access to SQLite databases.
  
- **sqlx**: An async SQL toolkit with compile-time checked queries that works well with Dioxus's async model.

- **Offline-First Architecture**: We can implement an offline-first architecture where data is stored locally in SQLite and synchronized with the backend when a connection is available.

### WebRTC and WebSocket Support

For real-time communication features:

- **WebSocket**: Dioxus can leverage Rust's `tokio-tungstenite` or `async-tungstenite` libraries for WebSocket communication, which integrate well with Dioxus's async model.

- **WebRTC**: For WebRTC functionality, we can use the `webrtc` crate or integrate with JavaScript WebRTC APIs through Dioxus's JavaScript interop capabilities.

## Project Structure

```
frontend-dioxus/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── app.rs                  # Main application component
│   ├── components/             # Reusable UI components
│   ├── features/               # Feature-specific modules
│   │   ├── auth/               # Authentication
│   │   ├── messaging/          # Messaging functionality
│   │   ├── calls/              # Audio/video calls
│   │   ├── circles/            # Circles feature
│   │   └── trees/              # Trees feature
│   ├── models/                 # Data models
│   ├── services/               # Service layer
│   │   ├── api.rs              # API client
│   │   ├── websocket.rs        # WebSocket client
│   │   ├── webrtc.rs           # WebRTC service
│   │   └── db.rs               # SQLite database service
│   ├── utils/                  # Utility functions
│   └── styles/                 # CSS styles
├── assets/                     # Static assets
├── Cargo.toml                  # Project dependencies
└── README.md                   # Project documentation
```

## Key Technical Advantages

### 1. Fullstack Capabilities

Dioxus offers a fullstack approach with server functions that work seamlessly across platforms. This allows us to:

- Write backend API calls directly in the frontend code
- Share types between frontend and backend
- Implement offline-first functionality with local SQLite storage

### 2. Asset Management

Dioxus's Manganis asset system provides:

- Automatic optimization of images and other assets
- Content hashing for better caching
- Cross-platform asset bundling

### 3. Modern Web Features

Dioxus supports modern web development patterns:

- Suspense boundaries for async operations
- HTML streaming for improved perceived performance
- Static site generation (SSG) and incremental static generation (ISG)

### 4. Error Handling

Dioxus 0.6+ supports Rust's native error handling with the `?` operator in components, event handlers, and tasks, making error management more idiomatic and less verbose.

## Getting Started

1. Install the Dioxus CLI:
   ```bash
   cargo install dioxus-cli@0.6.0
   ```

2. Run the development server:
   ```bash
   cd frontend-dioxus
   dx serve
   ```

3. For mobile development:
   ```bash
   dx serve --platform android  # For Android
   dx serve --platform ios      # For iOS (macOS only)
   ```

## SQLite Integration

Our SQLite integration uses a combination of `rusqlite` for the database operations and a custom service layer to handle synchronization with the backend:

```rust
// Example SQLite service
use rusqlite::{Connection, Result};
use crate::models::Message;

pub struct DbService {
    conn: Connection,
}

impl DbService {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("djibon.db")?;
        
        // Create tables if they don't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                content TEXT NOT NULL,
                sender_id TEXT NOT NULL,
                receiver_id TEXT NOT NULL,
                timestamp INTEGER NOT NULL,
                read INTEGER NOT NULL,
                synced INTEGER NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { conn })
    }
    
    pub fn save_message(&self, message: &Message) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO messages (id, content, sender_id, receiver_id, timestamp, read, synced)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [
                &message.id,
                &message.content,
                &message.sender_id,
                &message.receiver_id,
                &message.timestamp.to_string(),
                &(message.read as i32).to_string(),
                &(message.synced as i32).to_string(),
            ],
        )?;
        
        Ok(())
    }
    
    // More methods for CRUD operations...
}
```

## WebSocket and WebRTC Implementation

### WebSocket Service

```rust
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio::sync::mpsc;
use dioxus::prelude::*;

pub struct WebSocketService {
    tx: mpsc::Sender<String>,
}

impl WebSocketService {
    pub async fn new(url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();
        
        let (tx, mut rx) = mpsc::channel::<String>(100);
        
        // Handle outgoing messages
        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                write.send(Message::Text(message)).await.unwrap_or_else(|e| {
                    eprintln!("WebSocket send error: {}", e);
                });
            }
        });
        
        // Handle incoming messages
        tokio::spawn(async move {
            while let Some(message) = read.next().await {
                match message {
                    Ok(Message::Text(text)) => {
                        // Process incoming message
                        println!("Received: {}", text);
                    }
                    Ok(Message::Binary(data)) => {
                        // Handle binary data
                    }
                    Err(e) => {
                        eprintln!("WebSocket receive error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }
        });
        
        Ok(Self { tx })
    }
    
    pub async fn send_message(&self, message: String) -> Result<(), mpsc::error::SendError<String>> {
        self.tx.send(message).await
    }
}
```

### WebRTC Integration

For WebRTC, we can use the `webrtc` crate or integrate with JavaScript WebRTC APIs:

```rust
use dioxus::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn eval(s: &str) -> JsValue;
}

pub fn WebRTCComponent() -> Element {
    let video_ref = use_signal(|| None::<NodeRef>);
    
    let start_call = move |_| {
        if let Some(video_element) = video_ref() {
            // Initialize WebRTC using JavaScript interop
            let js = format!(
                r#"
                const pc = new RTCPeerConnection();
                navigator.mediaDevices.getUserMedia({{ video: true, audio: true }})
                    .then(stream => {{
                        stream.getTracks().forEach(track => pc.addTrack(track, stream));
                        document.getElementById('{}').srcObject = stream;
                    }});
                "#,
                video_element.get().unwrap().id()
            );
            
            let _ = eval(&js);
        }
    };
    
    rsx! {
        div {
            button { onclick: start_call, "Start Video Call" }
            video {
                id: "local-video",
                ref: video_ref,
                autoplay: true,
                muted: true
            }
        }
    }
}
```

## Conclusion

Dioxus offers a compelling alternative to Flutter for the DJibon PACT application, particularly for rapid prototyping and development. Its Rust-native approach, excellent hot-reloading capabilities, and seamless integration with SQLite make it an ideal choice for our project requirements.

By leveraging Dioxus, we can achieve faster development cycles, easier testing, and a more unified codebase across platforms, while still maintaining the ability to create beautiful, responsive user interfaces.
