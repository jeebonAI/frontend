use std::sync::Arc;
use dioxus::prelude::*;
use futures_util::{SinkExt, StreamExt};
use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console, WebSocket, MessageEvent, ErrorEvent, CloseEvent};

// Define a struct to hold connection status
#[derive(Clone, Debug, PartialEq)]
pub enum ConnectionStatus {
    Connected,
    Disconnected,
    Connecting,
    Failed,
}

// Create a service to manage WebSocket connections
#[derive(Clone)]
pub struct WebSocketService {
    status: UseState<ConnectionStatus>,
    ws: Option<WebSocket>,
    retry_count: UseState<u32>,
    max_retries: u32,
}

impl WebSocketService {
    pub fn new(cx: Scope) -> Self {
        let status = use_state(cx, || ConnectionStatus::Disconnected);
        let retry_count = use_state(cx, || 0);
        
        Self {
            status,
            ws: None,
            retry_count,
            max_retries: 5, // Maximum number of retry attempts
        }
    }
    
    pub fn connect(&mut self, url: &str) {
        let status = self.status.clone();
        let retry_count = self.retry_count.clone();
        let max_retries = self.max_retries;
        let url_owned = url.to_string();
        
        status.set(ConnectionStatus::Connecting);
        
        // Create a new WebSocket connection
        let ws = match WebSocket::new(url) {
            Ok(ws) => ws,
            Err(err) => {
                console::error_1(&format!("Failed to create WebSocket: {:?}", err).into());
                status.set(ConnectionStatus::Failed);
                return;
            }
        };
        
        // Set up event handlers
        let on_open = Closure::wrap(Box::new(move |_| {
            console::log_1(&"WebSocket connection established".into());
            status.set(ConnectionStatus::Connected);
            retry_count.set(0); // Reset retry count on successful connection
        }) as Box<dyn FnMut(JsValue)>);
        
        let status_err = status.clone();
        let retry_count_err = retry_count.clone();
        let url_retry = url_owned.clone();
        
        let on_error = Closure::wrap(Box::new(move |e: ErrorEvent| {
            // Don't log the error to console - we'll handle it gracefully
            status_err.set(ConnectionStatus::Failed);
            
            let current_retries = *retry_count_err.get();
            if current_retries < max_retries {
                // Implement exponential backoff for retries
                let backoff_ms = 1000 * (2_u32.pow(current_retries));
                let url_for_retry = url_retry.clone();
                let retry_count_inner = retry_count_err.clone();
                
                // Schedule a retry
                let timeout_callback = Closure::wrap(Box::new(move || {
                    retry_count_inner.set(current_retries + 1);
                    console::log_1(&format!("Retrying connection ({}/{})", 
                                          current_retries + 1, max_retries).into());
                    // This would normally reconnect, but we're just simulating it
                    // In a real implementation, you would call the connect method again
                }) as Box<dyn FnMut()>);
                
                web_sys::window()
                    .unwrap()
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        timeout_callback.as_ref().unchecked_ref(),
                        backoff_ms as i32,
                    )
                    .unwrap();
                
                timeout_callback.forget();
            } else {
                console::log_1(&"Max retries reached, using local data only".into());
            }
        }) as Box<dyn FnMut(ErrorEvent)>);
        
        let on_close = Closure::wrap(Box::new(move |e: CloseEvent| {
            console::log_1(&format!("WebSocket closed: {} - {}", e.code(), e.reason()).into());
            // Handle reconnection logic here if needed
        }) as Box<dyn FnMut(CloseEvent)>);
        
        // Set the event handlers
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));
        ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));
        
        // Keep the closures alive
        on_open.forget();
        on_error.forget();
        on_close.forget();
        
        self.ws = Some(ws);
    }
    
    pub fn status(&self) -> ConnectionStatus {
        (*self.status.get()).clone()
    }
}

// Repository pattern implementation for data access
#[derive(Clone)]
pub struct Repository<T: Serialize + for<'de> Deserialize<'de> + Clone + 'static> {
    websocket_service: WebSocketService,
    local_storage_key: String,
    data_type: std::marker::PhantomData<T>,
}

impl<T: Serialize + for<'de> Deserialize<'de> + Clone + 'static> Repository<T> {
    pub fn new(cx: Scope, websocket_service: WebSocketService, storage_key: &str) -> Self {
        Self {
            websocket_service,
            local_storage_key: storage_key.to_string(),
            data_type: std::marker::PhantomData,
        }
    }
    
    // Get data from server if connected, otherwise from local storage
    pub async fn get_data(&self, fallback_data: T) -> T {
        match self.websocket_service.status() {
            ConnectionStatus::Connected => {
                // In a real app, you would fetch from the server
                // For now, we'll just return the fallback and store it
                self.save_to_local_storage(&fallback_data);
                fallback_data
            },
            _ => {
                // Try to get from local storage
                self.get_from_local_storage().unwrap_or(fallback_data)
            }
        }
    }
    
    // Save data to local storage
    fn save_to_local_storage(&self, data: &T) -> Result<(), JsValue> {
        match serde_json::to_string(data) {
            Ok(json) => {
                LocalStorage::set(&self.local_storage_key, json).map_err(|e| {
                    JsValue::from_str(&format!("Failed to save to local storage: {:?}", e))
                })
            },
            Err(e) => Err(JsValue::from_str(&format!("Failed to serialize data: {:?}", e))),
        }
    }
    
    // Get data from local storage
    fn get_from_local_storage(&self) -> Option<T> {
        let json: Result<String, _> = LocalStorage::get(&self.local_storage_key);
        match json {
            Ok(json_str) => {
                match serde_json::from_str(&json_str) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        console::error_1(&format!("Failed to parse data from storage: {:?}", e).into());
                        None
                    }
                }
            },
            Err(_) => None,
        }
    }
}