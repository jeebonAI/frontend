use dioxus::prelude::*;

#[component]
pub fn Tree() -> Element {
    let family_tree_js = r###"
        document.addEventListener('DOMContentLoaded', function() {
            // Add CSS for family-chart
            const link = document.createElement('link');
            link.rel = 'stylesheet';
            link.href = 'https://unpkg.com/family-chart@0.0.0/dist/styles/family-chart.css';
            document.head.appendChild(link);
            
            // Load D3 first, then family-chart
            loadScript('https://unpkg.com/d3@6', function() {
                loadScript('https://unpkg.com/family-chart@0.0.0', function() {
                    try {
                        create(getData());
                    } catch (error) {
                        console.error('Error initializing family chart:', error);
                        const container = document.getElementById('FamilyChart');
                        if (container) {
                            container.innerHTML = '<div class="alert alert-danger">Error initializing family chart: ' + error.message + '</div>';
                        }
                    }
                });
            });
        });

        function loadScript(url, callback) {
            const script = document.createElement('script');
            script.type = url.includes('family-chart') ? 'module' : 'text/javascript';
            script.src = url;
            script.onload = callback;
            document.head.appendChild(script);
        }

        function create(data) {
            const cont = document.querySelector("#FamilyChart");
            const store = f3.createStore({
                data,
                node_separation: 250,
                level_separation: 150
            });
            const svg = f3.createSvg(cont);
            const Card = f3.elements.Card({
                store,
                svg,
                card_dim: {w:220, h:70, text_x:75, text_y:15, img_w:60, img_h:60, img_x:5, img_y:5},
                card_display: [d => `${d.data["first name"]} ${d.data["last name"]}`],
                mini_tree: true,
                link_break: false
            });

            store.setOnUpdate(props => f3.view(store.getTree(), svg, Card, props || {}));
            store.updateTree({initial: true});
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
                        "avatar": "https://static8.depositphotos.com/1009634/988/v/950/depositphotos_9883921-stock-illustration-no-user-profile-picture.jpg",
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
                        "avatar": ""
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
                        "avatar": ""
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
                        "avatar": ""
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
                        "avatar": ""
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
                        "avatar": ""
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
            class: "container my-3",
            // Tree Header
            div {
                class: "text-center mb-4",
                h2 { class: "display-6 fw-bold", "Family Tree" }
                p { class: "lead text-muted", "Explore and contribute to your family tree." }
            }
            // Tree Container
            div {
                class: "card shadow-sm",
                div {
                    class: "card-body",
                    div {
                        id: "FamilyChart",
                        class: "f3",
                        style: "width: 100%; height: 600px; background-color: rgb(33,33,33); color: #fff;"
                    }
                }
            }
            // Add script tag with the family tree JavaScript
            script {
                dangerous_inner_html: family_tree_js
            }
        }
    }
}
