use dioxus::prelude::*;

#[component]
pub fn Tree() -> Element {
    let family_tree_js = r###"
        // Execute immediately instead of waiting for DOMContentLoaded
        (function() {
            // Add CSS for family-chart
            const link = document.createElement('link');
            link.rel = 'stylesheet';
            link.href = 'https://unpkg.com/family-chart@0.0.0/dist/styles/family-chart.css';
            document.head.appendChild(link);
            
            // Load D3 first, then family-chart
            loadScript('https://unpkg.com/d3@6', function() {
                loadScript('https://unpkg.com/family-chart@0.0.0', function() {
                    // Small delay to ensure DOM is ready
                    setTimeout(() => {
                        try {
                            create(getData());
                            console.log("Family chart initialized");
                        } catch (error) {
                            console.error('Error initializing family chart:', error);
                            const container = document.getElementById('FamilyChart');
                            if (container) {
                                container.innerHTML = '<div class="alert alert-danger">Error initializing family chart: ' + error.message + '</div>';
                            }
                        }
                    }, 100);
                });
            });
        })();

        function loadScript(url, callback) {
            const script = document.createElement('script');
            script.type = url.includes('family-chart') ? 'module' : 'text/javascript';
            script.src = url;
            script.onload = callback;
            document.head.appendChild(script);
        }

        function create(data) {
            const cont = document.querySelector("#FamilyChart");
            if (!cont) {
                console.error("FamilyChart container not found");
                return;
            }
            
            // Clear any existing content
            cont.innerHTML = '';
            
            const store = f3.createStore({
                data,
                node_separation: 250,
                level_separation: 150
            });
            const svg = f3.createSvg(cont);
            const Card = f3.elements.Card({
                store,
                svg,
                card_dim: {w:220, h:100, text_x:75, text_y:15, img_w:60, img_h:60, img_x:5, img_y:5},
                card_display: [
                    d => `${d.data["first name"]} ${d.data["last name"]}`,
                    d => `${d.data["birthday"]}`,
                    d => `${d.data["role"] || ""}`,
                    d => `${d.data["info"] || ""}`
                ],
                mini_tree: true,
                link_break: false
            });

            store.setOnUpdate(props => f3.view(store.getTree(), svg, Card, props || {}));
            store.updateTree({initial: true});
        }

        // Generate SVG placeholder for missing images
        function generateAvatarSvg(name, gender) {
            const colors = {
                M: "#4285F4", // Blue for male
                F: "#EA4335", // Red for female
                O: "#34A853"  // Green for other
            };
            const color = colors[gender] || "#FBBC05"; // Yellow default
            const initials = name.split(' ').map(n => n[0]).join('').toUpperCase();
            
            return `data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" width="60" height="60" viewBox="0 0 60 60"><circle cx="30" cy="30" r="30" fill="${color}"/><text x="30" y="35" font-family="Arial" font-size="20" fill="white" text-anchor="middle">${initials}</text></svg>`;
        }

        function getData() {
            return [
                {
                    "id": "0",
                    "rels": {
                        "spouses": ["1"],
                        "children": ["2", "3"]
                    },
                    "data": {
                        "first name": "John",
                        "last name": "Doe",
                        "birthday": "1970",
                        "role": "Father",
                        "info": "Engineer",
                        "avatar": generateAvatarSvg("John Doe", "M"),
                        "gender": "M"
                    }
                },
                {
                    "id": "1",
                    "data": {
                        "gender": "F",
                        "first name": "Jane",
                        "last name": "Smith",
                        "birthday": "1972",
                        "role": "Mother",
                        "info": "Doctor",
                        "avatar": generateAvatarSvg("Jane Smith", "F")
                    },
                    "rels": {
                        "spouses": ["0"],
                        "children": ["2", "3"]
                    }
                },
                {
                    "id": "2",
                    "data": {
                        "gender": "M",
                        "first name": "Bob",
                        "last name": "Doe",
                        "birthday": "1995",
                        "role": "Son",
                        "info": "Teacher",
                        "avatar": generateAvatarSvg("Bob Doe", "M")
                    },
                    "rels": {
                        "father": "0",
                        "mother": "1",
                        "spouses": ["4"],
                        "children": ["5"]
                    }
                },
                {
                    "id": "3",
                    "data": {
                        "gender": "F",
                        "first name": "Alice",
                        "last name": "Doe",
                        "birthday": "1997",
                        "role": "Daughter",
                        "info": "Artist",
                        "avatar": generateAvatarSvg("Alice Doe", "F")
                    },
                    "rels": {
                        "father": "0",
                        "mother": "1"
                    }
                },
                {
                    "id": "4",
                    "data": {
                        "gender": "F",
                        "first name": "Carol",
                        "last name": "Johnson",
                        "birthday": "1996",
                        "role": "Daughter-in-law",
                        "info": "Architect",
                        "avatar": generateAvatarSvg("Carol Johnson", "F")
                    },
                    "rels": {
                        "spouses": ["2"],
                        "children": ["5"]
                    }
                },
                {
                    "id": "5",
                    "data": {
                        "gender": "M",
                        "first name": "David",
                        "last name": "Doe",
                        "birthday": "2020",
                        "role": "Grandson",
                        "info": "Student",
                        "avatar": generateAvatarSvg("David Doe", "M")
                    },
                    "rels": {
                        "father": "2",
                        "mother": "4"
                    }
                }
            ];
        }
    "###;

    rsx! {
        div {
            class: "container-fluid p-0 h-100",
            // Tree Container - full height and width with visible border for debugging
            div {
                id: "FamilyChart",
                class: "f3",
                style: "width: 100%; height: 100vh; background-color: #ffffff;"
            }
            // Add script tag with the family tree JavaScript
            script {
                dangerous_inner_html: family_tree_js
            }
        }
    }
}
