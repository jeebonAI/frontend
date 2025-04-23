# DJibon Frontend (Dioxus)

The frontend application for DJibon - a modern, secure, and platform-independent Personal Assistance and Communication Tool (PACT) built with Dioxus and Rust.

## Project Structure

```
frontend/
├── Cargo.toml                  # Rust dependencies and project metadata
├── Dioxus.toml                 # Dioxus configuration
├── index.html                  # HTML template for web builds
├── input.css                   # Tailwind input CSS (if using Tailwind)
├── tailwind.config.js          # Tailwind configuration (if using Tailwind)
├── assets/                     # Static assets
│   ├── images/                 # Image assets
│   ├── fonts/                  # Font files
│   └── styles/                 # CSS styles (including compiled Tailwind)
├── src/
│   ├── main.rs                 # Application entry point
│   ├── app.rs                  # Main application component
│   ├── routes.rs               # Route definitions
│   ├── components/             # Reusable UI components
│   │   ├── mod.rs              # Module exports
│   │   ├── layout/             # Layout components
│   │   ├── common/             # Common UI components
│   │   └── ...
│   ├── domains/                # Domain modules (business functionality)
│   │   ├── mod.rs              # Module exports
│   │   ├── auth/               # Authentication domain
│   │   │   ├── components/     # Auth-specific components
│   │   │   ├── signals/        # Auth-specific signal stores
│   │   │   └── hooks.rs        # Auth-specific hooks
│   │   ├── messaging/          # Messaging domain
│   │   │   ├── components/     # Messaging-specific components
│   │   │   ├── signals/        # Messaging-specific signal stores
│   │   │   └── hooks.rs        # Messaging-specific hooks
│   │   ├── calls/              # Calls domain
│   │   │   ├── components/     # Call-specific components
│   │   │   ├── signals/        # Call-specific signal stores
│   │   │   └── hooks.rs        # Call-specific hooks
│   │   ├── circles/            # Circles domain
│   │   └── trees/              # Trees domain
│   ├── hooks/                  # Shared custom hooks
│   │   ├── mod.rs
│   │   ├── use_auth.rs
│   │   ├── use_api.rs
│   │   └── use_offline.rs      # Hook for offline state management
│   ├── signals/                # Global signal stores
│   │   ├── mod.rs              # Signal exports
│   │   ├── app_state.rs        # Application-wide state
│   │   ├── theme.rs            # Theme state
│   │   └── notifications.rs    # Notification state
│   ├── services/               # Service layer
│   │   ├── mod.rs              # Service container and exports
│   │   ├── api/                # API services
│   │   │   ├── mod.rs
│   │   │   ├── client.rs       # HTTP client implementation
│   │   │   └── interceptor.rs  # Request/response interceptors
│   │   ├── websocket.rs        # WebSocket client
│   │   ├── webrtc.rs           # WebRTC service
│   │   ├── storage.rs          # Local storage service
│   │   ├── database/           # Database services
│   │   │   ├── mod.rs
│   │   │   ├── connection.rs   # SQLite connection management
│   │   │   ├── migrations.rs   # Database schema migrations
│   │   │   └── sync.rs         # Data synchronization logic
│   │   └── repositories/       # Data access repositories (gatekeepers)
│   │       ├── mod.rs
│   │       ├── user_repo.rs
│   │       ├── message_repo.rs
│   │       └── ...
│   ├── models/                 # Data models
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── message.rs
│   │   └── ...
│   ├── utils/                  # Utility functions
│   │   ├── mod.rs
│   │   ├── format.rs
│   │   ├── validation.rs
│   │   └── connectivity.rs     # Network connectivity detection
│   └── config/                 # Configuration
│       ├── mod.rs
│       └── app_config.rs
└── tests/                      # Integration tests
    ├── auth_tests.rs
    └── ...
```

## Architecture Overview

The DJibon frontend follows a domain-driven architecture with a focus on:

- **Separation of concerns**: Each domain is isolated and self-contained
- **Reusability**: Common components and hooks are shared across domains
- **Testability**: Components are designed to be easily testable
- **Maintainability**: Clear structure and organization
- **Offline-first**: Data is stored locally and synchronized with the backend when online

### Key Architectural Concepts

1. **Domain-driven organization**: Code is organized by business domains rather than technical concerns
2. **Signal-based state management**: Fine-grained reactivity using Dioxus Signals
3. **Custom hooks**: Encapsulate state and business logic
4. **Services**: Abstract external dependencies like API calls and storage
5. **Component composition**: Build complex UIs from simple, reusable components
6. **Repository pattern**: Abstracts data access operations for both local SQLite and remote API
7. **Offline-first approach**: Application works offline with local data and syncs when online

### Signal-Based State Management

Dioxus Signals provide a reactive state management system that enables fine-grained updates and efficient rendering. Our architecture leverages signals at multiple levels:

#### 1. Signal Fundamentals

Signals are reactive values that automatically track dependencies and trigger updates when values change:

```rust
// Basic signal usage
let counter = use_signal(|| 0);

// Reading from a signal
let current_value = *counter.read();

// Writing to a signal
counter.set(current_value + 1);

// Updating a signal based on current value
counter.update(|value| *value += 1);
```

#### 2. Domain-Specific Signal Stores

Each domain has a dedicated store that manages domain-specific state using signals:

```rust
// Example: Messaging domain store
pub struct MessagingStore {
    messages: Signal<Vec<Message>>,
    current_conversation: Signal<Option<Conversation>>,
    is_loading: Signal<bool>,
    // ...other state
}

impl MessagingStore {
    // Methods to interact with the state
    pub fn load_conversation(&self, id: String) { /* ... */ }
    pub fn send_message(&self, content: String) { /* ... */ }
    // ...other methods
}
```

These stores are typically created as global singletons using `once_cell`:

```rust
pub static MESSAGING_STORE: once_cell::sync::Lazy<MessagingStore> = 
    once_cell::sync::Lazy::new(|| MessagingStore::new());
```

#### 3. Signal-Based Hooks

Custom hooks use signals to provide reactive state and behavior to components:

```rust
pub fn use_auth() -> (Signal<Option<User>>, Signal<bool>, impl Fn(String, String) -> ()) {
    let current_user = use_signal(|| None::<User>);
    let is_loading = use_signal(|| false);
    
    let login = move |username: String, password: String| {
        // Authentication logic
    };
    
    (current_user, is_loading, login)
}
```

#### 4. Derived Signals

Computed values that automatically update when their dependencies change:

```rust
pub fn use_unread_message_count() -> Signal<usize> {
    let messages = MESSAGING_STORE.messages();
    
    use_memo(move || {
        messages.read().iter().filter(|msg| !msg.is_read).count()
    }, (messages,))
}
```

#### 5. Signal Effects

Side effects that run when signal values change:

```rust
pub fn use_sync_messages() {
    let is_online = NETWORK_SERVICE.is_online();
    let pending_messages = MESSAGING_STORE.pending_messages();
    
    use_effect(move || {
        if *is_online.read() && !pending_messages.read().is_empty() {
            MESSAGING_STORE.sync_pending_messages();
        }
    }, (is_online, pending_messages));
}
```

### Understanding the Project Structure

The project is organized into several key directories, each with a specific purpose:

#### 1. Domains (`src/domains/`)

Each domain represents a specific business area of the application. Domains are self-contained and include all the components, hooks, and models needed for that specific area.

**What goes in domains:**
- Domain-specific components
- Domain-specific hooks
- Domain-specific models
- Domain-specific repositories
- Domain-specific signal stores

**Examples:**
- `auth/` - Authentication and user management
- `messaging/` - Chat and messaging functionality
- `calls/` - Audio and video calling features
- `circles/` - User circles and connections
- `family/` - Family tree and relationships

#### 2. Components (`src/components/`)

Components are reusable UI elements that can be composed to build the interface. They focus on presentation rather than business logic.

**What goes in components:**
- Pure UI elements with no business logic
- Layout components (headers, footers, sidebars)
- Common UI elements (buttons, inputs, cards)
- Composite components built from smaller components

**Examples:**
- `layout/` - Page layouts, navigation components
- `common/` - Buttons, form inputs, cards, modals
- `feedback/` - Loaders, error messages, notifications

#### 3. Hooks (`src/hooks/`)

Hooks encapsulate reusable state logic and behavior. They abstract complex state management away from components.

**What goes in hooks:**
- Signal-based state management
- Data fetching and manipulation
- Side effects (API calls, local storage)
- Business logic that's independent of UI

**Examples:**
- `use_auth.rs` - Authentication state and methods
- `use_online_status.rs` - Network connectivity detection
- `use_theme.rs` - Theme management
- `use_form.rs` - Form state and validation

#### 4. Stores (`src/stores/`)

Stores manage application state using signals. They provide a centralized place for state management.

**What goes in stores:**
- Domain-specific signal stores
- Global application state
- Cross-cutting concerns (theme, notifications)

**Examples:**
- `auth_store.rs` - Authentication state
- `messaging_store.rs` - Messaging state
- `theme_store.rs` - Theme state
- `notification_store.rs` - Notification state

#### 5. Services (`src/services/`)

Services handle external interactions and technical concerns. They abstract away the details of how things work.

**What goes in services:**
- API clients
- WebSocket clients
- Local storage
- Database access
- Device APIs

**Examples:**
- `api_service.rs` - REST API client
- `websocket_service.rs` - WebSocket client
- `storage_service.rs` - Local storage
- `database_service.rs` - SQLite database
- `network_service.rs` - Network connectivity

#### 6. Repositories (`src/repositories/`)

Repositories abstract data access and coordinate between different data sources. They use signals to provide reactive data.

**What goes in repositories:**
- Data access logic
- Caching strategies
- Offline handling
- Synchronization logic

**Examples:**
- `user_repository.rs` - User data access
- `message_repository.rs` - Message data access
- `call_repository.rs` - Call data access
- `circle_repository.rs` - Circle data access

#### 7. Models (`src/models/`)

Models define the shape of data and provide methods for working with it.

**What goes in models:**
- Data structures
- Type definitions
- Validation logic
- Data transformation methods

**Examples:**
- `user.rs` - User model
- `message.rs` - Message model
- `circle.rs` - Circle model

### Data Flow and Component Interaction

Models flow through all layers of the application, with signals providing reactive updates:

```
┌──────────┐     ┌──────────┐     ┌─────────────┐     ┌───────────┐
│    UI    │◄───►│   Hooks  │◄───►│ Repositories │◄───►│ Services  │
└──────────┘     └──────────┘     └─────────────┘     └───────────┘
     │                │                  │                  │
     │                │                  │                  │
     └────────────────┴──────────────────┴──────────────────┘
                         Signal Stores
                              │
                              ▼
                            Models
```

**Typical data flow with signals:**

1. **Components** (UI) subscribe to signals and render data
2. **Hooks** provide signal-based state and methods to components
3. **Stores** manage domain-specific signals and business logic
4. **Repositories** provide reactive data access through signals
5. **Services** handle the technical details of API calls, database access, etc.
6. **Models** define the shape of data and provide methods for working with it

### Signal-Based Repository Pattern and Offline Strategy

The Repository pattern serves as the central gatekeeper for all data access in the application, enhanced with signals for reactivity:

1. **Signal-Based Repositories**:
   - Repositories expose data as signals
   - Components subscribe to these signals and automatically update when data changes
   - Changes to data are made through repository methods
   - Repositories handle caching, offline access, and synchronization

2. **Online Mode with Signals**:
   - UI subscribes to repository signals
   - User actions trigger repository methods
   - Repository fetches fresh data from Remote API
   - Repository updates local SQLite database
   - Repository updates signals with new data
   - UI automatically updates due to signal subscriptions

3. **Offline Mode with Signals**:
   - UI subscribes to repository signals
   - User actions trigger repository methods
   - Repository detects offline status
   - Repository fetches data from local SQLite database
   - Repository updates signals with local data
   - UI automatically updates due to signal subscriptions
   - Changes are queued for synchronization when online

4. **Synchronization with Signals**:
   - Network service exposes online status as a signal
   - When connection is restored, repositories detect online status change
   - Queued changes are sent to the server
   - Fresh data is pulled from the server
   - Local database is updated
   - Repository signals are updated with latest data
   - UI automatically updates due to signal subscriptions

### Signal-Based Implementation Examples

#### 1. Authentication Store

```rust
pub struct AuthStore {
    current_user: Signal<Option<User>>,
    is_loading: Signal<bool>,
    auth_error: Signal<Option<String>>,
    repository: AuthRepository,
}

impl AuthStore {
    pub fn new() -> Self {
        Self {
            current_user: Signal::new(None),
            is_loading: Signal::new(false),
            auth_error: Signal::new(None),
            repository: AuthRepository::new(),
        }
    }
    
    pub fn current_user(&self) -> Signal<Option<User>> {
        self.current_user
    }
    
    pub fn is_loading(&self) -> Signal<bool> {
        self.is_loading
    }
    
    pub fn auth_error(&self) -> Signal<Option<String>> {
        self.auth_error
    }
    
    pub fn login(&self, username: String, password: String) {
        self.is_loading.set(true);
        self.auth_error.set(None);
        
        match self.repository.login(username, password) {
            Ok(user) => {
                self.current_user.set(Some(user));
                self.is_loading.set(false);
            },
            Err(error) => {
                self.auth_error.set(Some(error.to_string()));
                self.is_loading.set(false);
            }
        }
    }
    
    pub fn logout(&self) {
        self.repository.logout();
        self.current_user.set(None);
    }
}

pub static AUTH_STORE: once_cell::sync::Lazy<AuthStore> = 
    once_cell::sync::Lazy::new(|| AuthStore::new());
```

#### 2. Message Repository

```rust
pub struct MessageRepository {
    api_service: ApiService,
    database_service: DatabaseService,
    network_service: &'static NetworkService,
}

impl MessageRepository {
    pub fn new() -> Self {
        Self {
            api_service: ApiService::new(),
            database_service: DatabaseService::new(),
            network_service: &NETWORK_SERVICE,
        }
    }
    
    pub fn get_messages(&self, conversation_id: &str) -> Signal<Vec<Message>> {
        let messages = Signal::new(Vec::new());
        
        // Load from local database first
        if let Ok(local_messages) = self.database_service.get_messages(conversation_id) {
            messages.set(local_messages);
        }
        
        // If online, fetch from API and update
        if *self.network_service.is_online().read() {
            if let Ok(remote_messages) = self.api_service.get_messages(conversation_id) {
                // Update local database
                self.database_service.save_messages(conversation_id, &remote_messages);
                // Update signal
                messages.set(remote_messages);
            }
        }
        
        messages
    }
    
    pub fn send_message(&self, conversation_id: &str, content: &str) -> Result<(), String> {
        let message = Message {
            id: uuid::Uuid::new_v4().to_string(),
            conversation_id: conversation_id.to_string(),
            content: content.to_string(),
            sender_id: AUTH_STORE.current_user().read().as_ref().map(|u| u.id.clone()).unwrap_or_default(),
            timestamp: chrono::Utc::now().timestamp(),
            is_sent: false,
            is_delivered: false,
            is_read: false,
        };
        
        // Save to local database
        self.database_service.save_message(&message)?;
        
        // If online, send to API
        if *self.network_service.is_online().read() {
            match self.api_service.send_message(&message) {
                Ok(_) => {
                    // Update message status
                    let mut updated_message = message.clone();
                    updated_message.is_sent = true;
                    self.database_service.update_message(&updated_message)?;
                },
                Err(_) => {
                    // Queue for later synchronization
                    self.database_service.queue_message_for_sync(&message)?;
                }
            }
        } else {
            // Queue for later synchronization
            self.database_service.queue_message_for_sync(&message)?;
        }
        
        Ok(())
    }
}
```

#### 3. Network Service

```rust
pub struct NetworkService {
    is_online: Signal<bool>,
}

impl NetworkService {
    pub fn new() -> Self {
        let is_online = Signal::new(Self::check_online_status());
        
        // Set up event listeners for online/offline events
        // This is simplified - in a real app you'd use proper event listeners
        
        Self {
            is_online,
        }
    }
    
    fn check_online_status() -> bool {
        // Check if the device is online
        true // Simplified for example
    }
    
    pub fn is_online(&self) -> Signal<bool> {
        self.is_online
    }
}

pub static NETWORK_SERVICE: once_cell::sync::Lazy<NetworkService> = 
    once_cell::sync::Lazy::new(|| NetworkService::new());
```

#### 4. Component Using Signals

```rust
pub fn MessageList(cx: Scope) -> Element {
    let conversation_id = use_state(cx, || "conversation-1".to_string());
    let messages = MESSAGING_STORE.get_messages(&conversation_id);
    let is_loading = MESSAGING_STORE.is_loading();
    
    cx.render(rsx! {
        div {
            class: "message-list",
            
            if *is_loading.read() {
                rsx! { div { class: "loading", "Loading messages..." } }
            } else if messages.read().is_empty() {
                rsx! { div { class: "empty", "No messages yet" } }
            } else {
                rsx! {
                    messages.read().iter().map(|message| {
                        rsx! {
                            MessageItem {
                                key: "{message.id}",
                                message: message.clone()
                            }
                        }
                    })
                }
            }
        }
    })
}
```

### Benefits of Signal-Based Architecture

1. **Performance**: Fine-grained reactivity means only components that depend on changed state will re-render
2. **Simplicity**: Signals provide a more intuitive mental model than complex state management libraries
3. **Consistency**: Using signals throughout your application provides a consistent approach to state management
4. **Testability**: Signal-based code is easier to test because state changes are explicit and trackable
5. **Offline-first**: Signals make it easier to implement offline-first functionality by providing reactive state that can be updated when connectivity changes

### Authentication and Security

The authentication system is designed to work both online and offline:

1. **Authentication Flow**:
   - User enters credentials
   - Credentials are validated against the backend
   - Backend returns JWT tokens (access and refresh)
   - Tokens are stored securely in the local database
   - Access token is used for subsequent API calls

2. **Token Management**:
   - Access token is used for all authenticated requests
   - Refresh token is used to obtain new tokens when access token expires
   - Tokens are stored securely using the platform's secure storage mechanisms

3. **Token Refresh**:
   - When access token expires (401 response), the frontend automatically:
     - Uses refresh token to request new tokens from `/auth/refresh` endpoint
     - Updates stored tokens
     - Retries the original request

#### Offline Authentication Flow

1. **Initial Authentication**:
   - User must authenticate at least once while online
   - Tokens and user data are stored in the local SQLite database
   - A longer-lived "offline token" is generated and stored

2. **Offline Login**:
   - When offline, the app validates the stored offline token
   - If valid, user is granted access to cached data
   - All changes are marked for synchronization when online

3. **Token Expiration Handling**:
   - If all tokens expire while offline, limited functionality is available
   - User is prompted to connect to the internet to re-authenticate
   - Cached data remains accessible in read-only mode

4. **Synchronization on Reconnection**:
   - When connection is restored, the app attempts to refresh tokens
   - If successful, offline changes are synchronized
   - If refresh fails, user is prompted to re-authenticate

### Implementation Details

1. **Auth Service** (`services/auth/auth_service.rs`):
   - Handles login, logout, and token refresh
   - Manages token storage and retrieval
   - Provides authentication state to the application

2. **Auth Repository** (`services/database/repositories/auth_repo.rs`):
   - Stores and retrieves authentication data from SQLite
   - Manages offline tokens and authentication state

3. **Auth Hook** (`hooks/use_auth.rs`):
   - Provides authentication state and methods to components
   - Handles authentication flow and redirects
   - Manages user session

4. **API Interceptor** (`services/api_interceptor.rs`):
   - Automatically adds authentication headers to requests
   - Handles token refresh on 401 responses
   - Queues requests when offline

5. **Connectivity Service** (`utils/connectivity.rs`):
   - Detects online/offline status
   - Triggers synchronization when connection is restored

### Security Considerations

- Tokens are stored in secure storage when available
- SQLite database is encrypted
- Sensitive operations require re-authentication
- Offline tokens have limited permissions

## Offline Database Implementation

### SQLite Schema Design

The frontend uses a SQLite database that mirrors essential parts of the backend SurrealDB schema, optimized for offline operation:

1. **Simplified Schema**: Only includes tables and fields needed for offline functionality
2. **ID Mapping**: Maps between SurrealDB record IDs and local SQLite IDs
3. **Sync Status Tracking**: Additional fields to track synchronization status

### Model Handling Strategy

We use a simplified approach with direct model usage (no DTOs):

1. **Single Model Definition**: Each entity has one model definition used throughout the application
2. **Client-Side Extensions**: Models include additional fields needed for client-side functionality
3. **Serialization Control**: Serde attributes control what gets sent to/from the API

```rust
// Example User model with both API and client-side fields
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    
    // Fields from API
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    
    // Client-side only fields (not sent to API)
    #[serde(skip_serializing, default)]
    pub is_synced: bool,
    #[serde(skip_serializing, default)]
    pub local_changes: Vec<Change>,
    #[serde(skip_serializing, default)]
    pub avatar_url: String,
}
```

This approach simplifies development by:
- Eliminating the need for mapping between different model types
- Reducing code duplication
- Making data flow more straightforward
- Keeping the codebase smaller and more maintainable

### Synchronization Strategy

The frontend SQLite database synchronizes with the backend using:

1. **Incremental Sync**: Only changed data is synchronized
2. **Change Tracking**: Each record includes last_modified timestamp and sync_status
3. **Conflict Resolution**: Deterministic strategy for handling conflicts
4. **Batch Operations**: Changes are grouped into efficient batches

#### Sync Process

1. **Outgoing Changes**:
   - Identify local changes with sync_status = "PENDING"
   - Group changes by entity type
   - Send changes to server in batches
   - Update sync_status based on server response

2. **Incoming Changes**:
   - Request changes since last_sync_timestamp
   - Apply changes to local database
   - Resolve conflicts using defined strategy
   - Update last_sync_timestamp

3. **Conflict Resolution Strategy**:
   - Server changes take precedence for most entities
   - Local changes take precedence for user-generated content
   - Merge strategy for non-conflicting field changes
   - Notification for user resolution when automatic resolution is not possible

### Optimizing Data Storage

To minimize duplication and storage requirements:

1. **Partial Data Storage**: Only store data relevant to the current user
2. **Lazy Loading**: Load additional data on demand when online
3. **TTL Policies**: Automatically expire old data based on usage patterns
4. **Compression**: Compress large text fields and binary data

### Implementation Components

1. **Database Service** (`services/database/mod.rs`):
   - Manages SQLite connection and migrations
   - Provides transaction support
   - Handles database encryption

2. **Sync Service** (`services/database/sync.rs`):
   - Orchestrates synchronization process
   - Implements conflict resolution
   - Tracks sync state

3. **Repositories** (`services/database/repositories/`):
   - Domain-specific data access
   - Implements CRUD operations for both online and offline modes
   - Handles data mapping between API and SQLite

4. **Models** (`models/`):
   - Defines frontend data structures
   - Includes serialization/deserialization logic
   - Maps between API responses and local storage

This approach minimizes duplication while ensuring the frontend can function effectively offline.

## Real-time Communication Architecture

The frontend implements multiple communication channels to support different types of interactions:

### Communication Channels

1. **HTTP/REST API**
   - **Used for:**
     - Authentication
     - Resource CRUD operations
     - Data synchronization
     - Configuration
   - **Implementation:**
     - Rust-based HTTP client
     - Request interceptors for authentication
     - Offline request queuing
     - Retry mechanisms

2. **WebSocket**
   - **Used for:**
     - Real-time messaging
     - Presence updates (online/offline status)
     - Circle activity notifications
     - Family tree updates
     - Notifications
     - Live updates
     - WebRTC signaling
   - **Implementation:**
     - Persistent WebSocket connection
     - Automatic reconnection
     - Message queuing during disconnection
     - Event-based architecture
     - Message queuing for offline support
     - Typed message handling

3. **WebRTC**
   - **Used for:**
     - Audio/video calls
     - Screen sharing
     - Peer-to-peer file transfers
   - **Implementation:**
     - WebRTC signaling through WebSockets
     - STUN/TURN server integration for NAT traversal
     - Media stream handling
     - Fallback to TURN relay when direct connection fails
     - End-to-end encryption for all communications

### Message Types

| Channel | Direction | Message Type | Purpose |
|---------|-----------|--------------|---------|
| REST    | Client → Server | Authentication | Login, register, token refresh |
| REST    | Client → Server | Resource CRUD | Create, read, update, delete resources |
| REST    | Client ← Server | Resource Data | Fetch resources and data |
| WS      | Client ← Server | Notification | System and user notifications |
| WS      | Client ← Server | Presence | User online/offline status |
| WS      | Client ← Server | Message | Real-time message delivery |
| WS      | Bidirectional | Call Signaling | WebRTC connection establishment |
| WebRTC  | Bidirectional | Media Stream | Audio/video call data |

### Domain-Specific Communication

Each domain uses the appropriate communication channel based on its requirements:

#### Authentication Domain
- **Primary Channel:** REST API
- **Use Cases:** Login, registration, token refresh
- **Offline Strategy:** Cached tokens, offline validation

#### Messaging Domain
- **Primary Channels:** WebSocket + REST API
- **Use Cases:** 
  - WebSocket: Real-time message delivery, typing indicators, read receipts
  - REST API: Message history, conversation metadata
- **Offline Strategy:** Store messages locally, queue outgoing messages

#### Calls Domain
- **Primary Channels:** WebRTC + WebSocket
- **Use Cases:**
  - WebRTC: Audio/video streaming, screen sharing
  - WebSocket: Call signaling, presence updates
- **Offline Strategy:** Detect connectivity issues, provide feedback

#### Circles Domain
- **Primary Channels:** REST API + WebSocket
- **Use Cases:**
  - REST API: Circle management, member lists
  - WebSocket: Real-time updates, activity notifications
- **Offline Strategy:** Cache circle data, queue changes

#### Trees Domain
- **Primary Channel:** REST API
- **Use Cases:** Family tree management, relationship data
- **Offline Strategy:** Cache tree data, queue changes

### WebSocket Implementation

The WebSocket client maintains a persistent connection to the server:

```rust
pub struct WebSocketClient {
    connection: Option<WebSocketConnection>,
    event_bus: EventBus,
    reconnect_timer: Option<Timer>,
    auth_service: Arc<AuthService>,
    connection_status: ConnectionStatus,
}

impl WebSocketClient {
    // Connect to WebSocket server with authentication
    pub async fn connect(&mut self) -> Result<(), WebSocketError> {
        let token = self.auth_service.get_access_token().await?;
        let url = format!("{}/ws?token={}", API_BASE_URL, token);
        
        self.connection = Some(WebSocketConnection::connect(&url).await?);
        self.start_message_handler();
        self.connection_status = ConnectionStatus::Connected;
        
        Ok(())
    }
    
    // Send message to server
    pub async fn send_message(&self, message: WebSocketMessage) -> Result<(), WebSocketError> {
        if let Some(conn) = &self.connection {
            conn.send(message).await?;
            Ok(())
        } else {
            Err(WebSocketError::NotConnected)
        }
    }
    
    // Handle incoming messages
    fn start_message_handler(&self) {
        // Implementation details
    }
    
    // Reconnect logic
    async fn reconnect(&mut self) {
        // Implementation details
    }
}
```

### WebRTC Implementation

The WebRTC client handles peer-to-peer connections:

```rust
pub struct WebRTCClient {
    peer_connection: Option<RTCPeerConnection>,
    signaling_service: Arc<WebSocketClient>,
    media_devices: MediaDevices,
    data_channels: HashMap<String, RTCDataChannel>,
}

impl WebRTCClient {
    // Initialize a call
    pub async fn initiate_call(&mut self, peer_id: &str) -> Result<CallSession, WebRTCError> {
        // Create peer connection
        self.peer_connection = Some(self.create_peer_connection().await?);
        
        // Add media tracks
        let local_stream = self.media_devices.get_user_media().await?;
        self.add_tracks(local_stream).await?;
        
        // Create and send offer
        let offer = self.peer_connection.as_ref().unwrap().create_offer().await?;
        self.signaling_service.send_message(WebSocketMessage::CallSignal {
            peer_id: peer_id.to_string(),
            signal: Signal::Offer(offer),
        }).await?;
        
        // Return call session
        Ok(CallSession::new(peer_id, self.peer_connection.clone()))
    }
    
    // Handle incoming call
    pub async fn handle_incoming_call(&mut self, peer_id: &str, offer: RTCSessionDescription) 
        -> Result<CallSession, WebRTCError> {
        // Implementation details
        // ...
    }
    
    // Create data channel
    pub async fn create_data_channel(&mut self, label: &str) -> Result<(), WebRTCError> {
        // Implementation details
        // ...
    }
}
```

### Offline Handling

The communication architecture handles offline scenarios gracefully:

1. **Connection Monitoring**:
   - Detect network status changes
   - Gracefully handle disconnections
   - Queue messages during offline periods

2. **Reconnection Strategy**:
   - Exponential backoff for reconnection attempts
   - Session resumption when possible
   - State reconciliation after reconnection

3. **Fallback Mechanisms**:
   - REST API fallback when WebSocket is unavailable
   - TURN server relay when direct WebRTC connection fails
   - Local cache for frequently accessed data

## Media Management Architecture

### Media Types and Use Cases

DJibon supports various media types across different domains:

| Media Type | Use Cases |
|------------|-----------|
| Images     | Profile pictures, message attachments, family records |
| Videos     | Message attachments, call recordings, family records |
| Audio      | Voice messages, call recordings, voice notes |
| Documents  | Shared files, family records, verification documents |

### Media Storage Strategy

The application uses a multi-tiered approach to media storage:

1. **Remote Storage**:
   - Cloud storage for permanent media (S3-compatible)
   - CDN integration for efficient delivery
   - Access control based on user permissions

2. **Local Cache**:
   - SQLite BLOB storage for frequently accessed media
   - File system cache for larger media files
   - Intelligent prefetching based on usage patterns
   - Cache expiration policies based on media type and size

3. **Media Processing**:
   - Client-side image resizing and compression
   - Thumbnail generation for gallery views
   - Progressive loading for large media files
   - Format conversion for compatibility

### Media Repository

The Media Repository acts as the central coordinator for all media operations:

```rust
// Example Media Repository interface
pub trait MediaRepository {
    // Upload methods
    async fn upload_media(&self, file: File, media_type: MediaType, metadata: MediaMetadata) -> Result<MediaRecord>;
    
    // Retrieval methods
    async fn get_media(&self, media_id: &str) -> Result<MediaContent>;
    async fn get_thumbnail(&self, media_id: &str, size: ThumbnailSize) -> Result<MediaContent>;
    
    // Management methods
    async fn delete_media(&self, media_id: &str) -> Result<()>;
    async fn update_metadata(&self, media_id: &str, metadata: MediaMetadata) -> Result<MediaRecord>;
    
    // Offline support
    async fn prefetch_media(&self, media_ids: Vec<String>) -> Result<()>;
    fn get_cache_status(&self, media_id: &str) -> CacheStatus;
}
```

### Media Components

The UI includes specialized components for media handling:

1. **MediaViewer**: Displays different media types with appropriate controls
2. **MediaUploader**: Handles media selection and upload with progress indication
3. **MediaGallery**: Displays collections of media with filtering and sorting
4. **MediaThumbnail**: Optimized thumbnail display with lazy loading

### Offline Media Access

The application implements several strategies for offline media access:

1. **Automatic Caching**: Frequently accessed media is automatically cached
2. **Manual Prefetching**: Users can download media for offline access
3. **Progressive Enhancement**: Fallback to thumbnails when full media is unavailable
4. **Upload Queuing**: Media uploads are queued when offline and processed when online

## Getting Started

1. Install the Dioxus CLI:
   ```bash
   cargo install dioxus-cli
   ```

2. Run the development server:
   ```bash
   dx serve
   ```

3. Build for production:
   ```bash
   dx build --release
   ```

## Development Guidelines

- Each domain should be self-contained with its own components, pages, and hooks
- Shared functionality should be extracted to the appropriate shared directory
- Follow Rust naming conventions (snake_case for functions and variables, CamelCase for types)
- Use Rust's module system with explicit exports in mod.rs files
- Keep components small and focused on a single responsibility
- All data access should go through repositories to abstract the data source
- Always consider offline scenarios when implementing features

### Models and Data Flow

The DJibon frontend uses a simplified model architecture that aligns with our backend approach:

#### Direct Model Usage (No DTOs)

We've opted for a streamlined approach without Data Transfer Objects (DTOs):

- **Single Model Definition**: Each entity has one model definition used throughout the application
- **Serialization Attributes**: Models use Serde attributes to handle API serialization/deserialization
- **Optional Fields**: Models account for differences between creation, updates, and retrieval with optional fields

This approach:
- Reduces code duplication
- Simplifies maintenance
- Keeps the codebase smaller
- Makes data flow more straightforward

Example model:

```rust
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(default)]
    pub email_verified: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    #[serde(default)]
    pub status: String,
    
    // Client-side only fields
    #[serde(skip_serializing, default)]
    pub is_online: bool,
}
```

With this approach, the same model is used for:
- API requests and responses
- Local database storage
- UI state management

### Domains vs. Shared Hooks: Understanding the Distinction

#### Domain-Specific vs. Shared Hooks

Our architecture has two types of hooks:

1. **Domain-Specific Hooks** (`src/domains/*/hooks.rs`):
   - Live inside domain directories
   - Handle business logic specific to that domain
   - Only used within that domain
   - Example: `domains/messaging/hooks.rs` might contain `use_conversation_list()` and `use_message_composer()`

2. **Shared Hooks** (`src/hooks/`):
   - Live in the shared hooks directory
   - Used across multiple domains
   - Handle cross-cutting concerns
   - Example: `hooks/use_auth.rs` is used by multiple domains that need authentication

#### Business Logic Distribution

Business logic is distributed across several places:

1. **Hooks** (both domain-specific and shared):
   - Contain most UI-related business logic
   - Handle state management and UI updates
   - Coordinate between UI and data layer

2. **Repositories** (`src/services/repositories/`):
   - Contain data access business logic
   - Handle caching, offline strategies, and sync
   - Coordinate between different data sources

3. **Models** (`src/models/`):
   - Contain entity-specific business logic
   - Handle validation and data transformation
   - Provide methods for working with entity data

4. **Services** (`src/services/`):
   - Contain technical implementation details
   - Handle external communication protocols
   - Provide interfaces for data access

This distribution ensures that:
- UI-related logic stays close to the UI (in hooks)
- Data-related logic stays close to the data (in repositories and models)
- Technical details are abstracted away (in services)

### Suspense and Resource Management

Dioxus provides a suspense system for handling asynchronous operations elegantly:

```rust
// Resource creation
let user_resource = use_resource(move || async move {
    api_client.fetch_user(user_id).await
});

// Usage with suspense
rsx! {
    Suspense {
        fallback: rsx! { LoadingSpinner {} },
        // Resource automatically unwraps inside suspense
        UserProfile { user: user_resource.read() }
    }
}
```

This approach:
- Separates data fetching from rendering
- Provides consistent loading states
- Handles errors gracefully
- Works well with our signal-based architecture

### Server Functions

For server-client communication, Dioxus supports server functions that can be called directly from the frontend:

```rust
#[server]
async fn fetch_user_data(user_id: String) -> Result<User, ServerFnError> {
    // Server-side implementation
    Ok(db.get_user(user_id).await?)
}

// Client component
fn UserProfile() -> Element {
    let user = use_server_fn::<FetchUserData>();
    let result = use_resource(move || async move {
        user.call(current_user_id).await
    });
    
    // Render with result
}
```

This approach:
- Reduces boilerplate for API calls
- Ensures type safety across client and server
- Integrates with our authentication system
- Works alongside our existing API services

### Fermi for Global State

While our architecture primarily uses Dioxus Signals, Fermi provides an alternative approach for global state:

```rust
// Define an atom
static USER: Atom<Option<User>> = |_| None;

// Read from atom
let user = use_read(USER);

// Write to atom
use_set(USER)(Some(new_user));

// Subscribe to changes
let user = use_atom_state(USER);
```

Fermi can complement our signal-based approach for certain global state needs.

### Structured Logging with Tracing

Our architecture leverages Tokio's tracing for structured logging:

```rust
// Initialize tracing
tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();

// Use throughout the application
#[instrument(skip(password))]
async fn login(username: String, password: String) -> Result<User, Error> {
    tracing::info!("Login attempt for user: {}", username);
    // Implementation
}
```

Benefits:
- Structured logs for better debugging
- Span-based request tracking
- Performance insights
- Works across all platforms

### Concurrency Management

For background tasks and concurrent operations, we use Tokio's sync primitives:

```rust
// Shared state with RwLock
let shared_state = Arc::new(RwLock::new(AppState::default()));

// Background task with channels
let (tx, mut rx) = mpsc::channel(100);
tokio::spawn(async move {
    while let Some(message) = rx.recv().await {
        // Process message
    }
});

// Send messages
tx.send(message).await?;
```

This approach:
- Safely shares state between tasks
- Enables efficient background processing
- Integrates with our signal-based architecture

### WebAssembly Optimizations

For web platforms, we optimize our WebAssembly bundle:

```toml
# In Cargo.toml
[profile.release]
opt-level = 'z'
lto = true
codegen-units = 1
panic = 'abort'

[dependencies]
wasm-bindgen = "0.2"
web-sys = { version = "0.3", features = [...] }
```

Our architecture:
- Minimizes WASM bundle size
- Uses web-sys for direct browser API access
- Implements platform-specific optimizations
- Leverages web workers for CPU-intensive tasks

### Cross-Platform Architecture

Our Dioxus application targets multiple platforms with platform-specific optimizations:

#### Platform Detection and Adaptation

```rust
#[cfg(target_arch = "wasm32")]
fn initialize_storage() -> impl Storage {
    WebStorage::new()
}

#[cfg(not(target_arch = "wasm32"))]
fn initialize_storage() -> impl Storage {
    FileSystemStorage::new()
}

// Usage is platform-agnostic
let storage = initialize_storage();
```

#### Platform-Specific Features

- **Web**: Service workers for offline support, IndexedDB for storage
- **Desktop**: File system access, system notifications, tray integration
- **Mobile**: Touch optimizations, responsive layouts, native API access

This approach ensures consistent behavior while leveraging platform capabilities.

### SQLite Database Access

For our offline-first database, we use rusqlite with SQLite:

```rust
// Database operations with rusqlite
let user = conn.query_row(
    "SELECT * FROM users WHERE id = ?",
    params![user_id],
    |row| {
        Ok(User {
            id: row.get(0)?,
            name: row.get(1)?,
            // other fields
        })
    }
)?;

// Transaction support
let tx = conn.transaction()?;
// Multiple operations...
tx.commit()?;
```

#### Web Platform SQLite Strategy

For web platforms, we use `sqlite-wasm` which compiles SQLite to WebAssembly:

```rust
#[cfg(target_arch = "wasm32")]
fn initialize_database() -> impl DatabaseConnection {
    // Use sqlite-wasm which stores data in IndexedDB
    WebSqliteConnection::new("djibon.db")
}

#[cfg(not(target_arch = "wasm32"))]
fn initialize_database() -> impl DatabaseConnection {
    // Use native SQLite for desktop/mobile
    NativeSqliteConnection::new("djibon.db")
}
```

The web version stores the SQLite database in IndexedDB, which:
- Persists between sessions
- Provides significant storage capacity
- Supports our offline-first approach
- Maintains compatibility with our SQLite schema

## Dioxus vs. Established Frameworks

Comparing our Dioxus-based architecture to established frameworks like Flutter reveals several strengths and considerations:

### Advantages of Our Approach

1. **Rust's Safety Guarantees**
   - Memory safety without garbage collection
   - Thread safety with compile-time verification
   - Fewer runtime crashes and edge cases
   - More predictable performance characteristics

2. **Unified Language Stack**
   - Same language (Rust) for frontend and backend
   - Shared types and validation logic
   - Reduced context switching for developers
   - Better type safety across the entire application

3. **WebAssembly Performance**
   - More efficient web execution compared to Flutter's JS bridge
   - Smaller bundle sizes for web deployment
   - Better CPU-intensive operation performance
   - Direct DOM access when needed

4. **Modern Reactive Paradigm**
   - Fine-grained reactivity with signals
   - More efficient updates than Flutter's widget rebuilds
   - Less boilerplate than Flutter's StatefulWidget pattern
   - More intuitive mental model for state management

5. **Offline-First Architecture**
   - Robust SQLite integration across all platforms
   - Consistent database API regardless of platform
   - Strong data synchronization capabilities
   - Better handling of intermittent connectivity

### Considerations and Challenges

1. **Ecosystem Maturity**
   - Flutter has a larger ecosystem of packages and plugins
   - More established patterns and best practices
   - Larger community for support and troubleshooting
   - More comprehensive documentation

2. **UI Component Library**
   - Flutter has a more comprehensive widget library
   - More consistent cross-platform rendering
   - Better accessibility support out of the box
   - More mature animation system

3. **Development Velocity**
   - Flutter's hot reload can be faster in some scenarios
   - Dart may have a gentler learning curve than Rust
   - More developers familiar with Flutter/Dart than Dioxus/Rust
   - More tooling support (IDE integration, debugging)

4. **Platform Integration**
   - Flutter has more mature native platform integration
   - Better support for platform-specific features
   - More comprehensive plugin ecosystem
   - More tested on various device configurations

### Conclusion

Our Dioxus-based architecture offers significant advantages in terms of:
- Performance and efficiency
- Type safety and reliability
- Language consistency
- Modern reactive programming

While Flutter currently has advantages in ecosystem maturity and UI component richness, our approach is better positioned for:
- Applications requiring high performance
- Projects where type safety is critical
- Teams already invested in Rust
- Applications with significant offline capabilities

As Dioxus continues to mature (currently at version 0.6+), many of the gaps compared to Flutter are narrowing. For our specific use case—a personal communication tool with offline capabilities—the benefits of our approach outweigh the limitations, especially considering our team's expertise in Rust and the importance of data integrity in our application.
