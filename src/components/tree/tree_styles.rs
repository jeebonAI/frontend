use dioxus::prelude::*;

#[component]
pub fn include_styles() -> Element {
    rsx! {
        style { 
            ".family-tree-container {{ padding: 20px; }}"
            ".family-member {{ 
                display: flex; 
                align-items: center; 
                padding: 10px; 
                border: 1px solid #ddd; 
                border-radius: 5px; 
                margin-bottom: 10px; 
                cursor: pointer; 
                position: relative; 
            }}"
            ".central {{ 
                background-color: #f8f9fa; 
                border-color: #0d6efd; 
            }}"
            ".member-avatar {{ 
                width: 40px; 
                height: 40px; 
                border-radius: 50%; 
                background: #e9ecef; 
                display: flex; 
                align-items: center; 
                justify-content: center; 
                margin-right: 15px; 
            }}"
            ".member-info {{ flex: 1; }}"
            ".member-name {{ font-weight: bold; }}"
            ".member-year, .member-relation {{ 
                font-size: 0.85rem; 
                color: #6c757d; 
            }}"
            ".member-options {{ 
                position: absolute; 
                right: 10px; 
                top: 50%; 
                transform: translateY(-50%); 
                z-index: 10; 
            }}"
        }
    }
}
