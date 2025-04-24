use dioxus::prelude::*;

#[component]
pub fn Tree() -> Element {
    let cytoscape_js = r###"
        function loadScript(url, callback) {
            const script = document.createElement('script');
            script.type = 'text/javascript';
            script.src = url;
            script.onload = callback;
            document.head.appendChild(script);
        }

        loadScript('https://cdnjs.cloudflare.com/ajax/libs/graphlib/2.1.8/graphlib.min.js', function() {
            loadScript('https://cdnjs.cloudflare.com/ajax/libs/cytoscape/3.26.0/cytoscape.min.js', function() {
                loadScript('https://cdn.jsdelivr.net/npm/dagre@0.8.5/dist/dagre.min.js', function() {
                    loadScript('https://cdn.jsdelivr.net/npm/cytoscape-dagre@2.5.0/cytoscape-dagre.min.js', function() {
                        
                        // Initialize the graph
                        const cy = cytoscape({
                            container: document.getElementById('cyto-container'),
                            elements: [
                                // People nodes
                                { data: { id: 'p1', name: 'John Doe', gender: 'male', generation: 1 } },
                                { data: { id: 'p2', name: 'Jane Smith', gender: 'female', generation: 1 } },
                                { data: { id: 'p3', name: 'Mary Brown', gender: 'female', generation: 1 } },
                                { data: { id: 'c1', name: 'Alice Doe', gender: 'female', generation: 2 } },
                                { data: { id: 'c2', name: 'Bob Doe', gender: 'male', generation: 2 } },
                                
                                // Relationship nodes (invisible)
                                { data: { id: 'r1', type: 'marriage', generation: 1 } },
                                { data: { id: 'r2', type: 'parent-child', generation: 1.5 } },
                                { data: { id: 'r3', type: 'parent-child', generation: 1.5 } },
                                
                                // Marriage edges
                                { data: { id: 'e1', source: 'p1', target: 'r1', type: 'spouse' } },
                                { data: { id: 'e2', source: 'p2', target: 'r1', type: 'spouse' } },
                                
                                // Parent-child edges
                                { data: { id: 'e3', source: 'r1', target: 'r2', type: 'parent-child' } },
                                { data: { id: 'e4', source: 'r2', target: 'c1', type: 'child' } },
                                { data: { id: 'e5', source: 'r1', target: 'r3', type: 'parent-child' } },
                                { data: { id: 'e6', source: 'r3', target: 'c2', type: 'child' } },
                                
                                // Step-parent relationship
                                { data: { id: 'e7', source: 'p3', target: 'c2', type: 'step' } }
                            ],
                            style: [
                                {
                                    selector: 'node[gender]',
                                    style: {
                                        'label': 'data(name)',
                                        'text-valign': 'bottom',
                                        'text-margin-y': 10,
                                        'width': 30,
                                        'height': 30,
                                        'background-color': function(ele) {
                                            return ele.data('gender') === 'male' ? '#007bff' : '#ff69b4';
                                        },
                                        'font-size': 14,
                                        'font-weight': 'bold',
                                        'text-wrap': 'wrap',
                                        'border-width': 2,
                                        'border-color': '#333'
                                    }
                                },
                                {
                                    selector: 'node[type]',
                                    style: {
                                        'width': 10,
                                        'height': 10,
                                        'background-color': '#aaa',
                                        'opacity': 0
                                    }
                                },
                                {
                                    selector: 'edge[type="spouse"]',
                                    style: { 
                                        'line-style': 'solid',
                                        'width': 3,
                                        'line-color': '#888',
                                        'curve-style': 'straight'
                                    }
                                },
                                {
                                    selector: 'edge[type="parent-child"]',
                                    style: { 
                                        'line-style': 'solid',
                                        'width': 3,
                                        'line-color': '#888',
                                        'curve-style': 'straight'
                                    }
                                },
                                {
                                    selector: 'edge[type="child"]',
                                    style: { 
                                        'line-style': 'solid',
                                        'width': 3,
                                        'line-color': '#888',
                                        'curve-style': 'straight',
                                        'target-arrow-shape': 'triangle',
                                        'target-arrow-color': '#888'
                                    }
                                },
                                {
                                    selector: 'edge[type="step"]',
                                    style: { 
                                        'line-style': 'dotted',
                                        'width': 2,
                                        'line-color': '#888',
                                        'curve-style': 'bezier',
                                        'target-arrow-shape': 'triangle',
                                        'target-arrow-color': '#888',
                                        'line-dash-pattern': [6, 3]
                                    }
                                }
                            ],
                            layout: {
                                name: 'dagre',
                                rankDir: 'TB',
                                rankSep: 100,
                                nodeSep: 50,
                                ranker: 'network-simplex',
                                align: 'UL',
                                // Use generations to enforce proper hierarchy
                                rankFn: function(node) {
                                    return node.data('generation');
                                }
                            },
                            zoom: 1,
                            minZoom: 0.5,
                            maxZoom: 2,
                            wheelSensitivity: 0.2
                        });

                        // Add tooltips to show relationship types
                        cy.on('mouseover', 'edge', function(e) {
                            const edge = e.target;
                            const type = edge.data('type');
                            
                            // Create tooltip
                            let tooltip = document.getElementById('cy-tooltip');
                            if (!tooltip) {
                                tooltip = document.createElement('div');
                                tooltip.id = 'cy-tooltip';
                                tooltip.style.position = 'absolute';
                                tooltip.style.backgroundColor = 'rgba(0,0,0,0.75)';
                                tooltip.style.color = 'white';
                                tooltip.style.padding = '5px 10px';
                                tooltip.style.borderRadius = '3px';
                                tooltip.style.fontSize = '12px';
                                tooltip.style.pointerEvents = 'none';
                                tooltip.style.zIndex = '999';
                                document.body.appendChild(tooltip);
                            }
                            
                            // Set tooltip content based on relationship type
                            let relationshipText = '';
                            switch(type) {
                                case 'spouse': relationshipText = 'Spouse'; break;
                                case 'biological': relationshipText = 'Biological Parent'; break;
                                case 'step': relationshipText = 'Step Parent'; break;
                                case 'child': relationshipText = 'Child'; break;
                                default: relationshipText = type.charAt(0).toUpperCase() + type.slice(1);
                            }
                            
                            tooltip.textContent = relationshipText;
                            
                            // Position tooltip near mouse
                            const renderedPosition = edge.renderedMidpoint();
                            const containerRect = cy.container().getBoundingClientRect();
                            
                            tooltip.style.left = (containerRect.left + renderedPosition.x) + 'px';
                            tooltip.style.top = (containerRect.top + renderedPosition.y - 30) + 'px';
                            tooltip.style.display = 'block';
                        });
                        
                        cy.on('mouseout', 'edge', function() {
                            const tooltip = document.getElementById('cy-tooltip');
                            if (tooltip) {
                                tooltip.style.display = 'none';
                            }
                        });

                        // Add legend
                        const legend = document.createElement('div');
                        legend.style.position = 'absolute';
                        legend.style.bottom = '10px';
                        legend.style.right = '10px';
                        legend.style.backgroundColor = 'rgba(255,255,255,0.9)';
                        legend.style.padding = '10px';
                        legend.style.border = '1px solid #ddd';
                        legend.style.borderRadius = '5px';
                        legend.style.fontSize = '12px';
                        
                        legend.innerHTML = `
                            <div style="font-weight: bold; margin-bottom: 5px;">Legend</div>
                            <div style="display: flex; align-items: center; margin-bottom: 3px;">
                                <div style="width: 15px; height: 15px; border-radius: 50%; background-color: #007bff; margin-right: 5px;"></div>
                                <span>Male</span>
                            </div>
                            <div style="display: flex; align-items: center; margin-bottom: 3px;">
                                <div style="width: 15px; height: 15px; border-radius: 50%; background-color: #ff69b4; margin-right: 5px;"></div>
                                <span>Female</span>
                            </div>
                            <div style="display: flex; align-items: center; margin-bottom: 3px;">
                                <div style="width: 30px; height: 2px; background-color: #888; margin-right: 5px;"></div>
                                <span>Marriage</span>
                            </div>
                            <div style="display: flex; align-items: center; margin-bottom: 3px;">
                                <div style="width: 30px; height: 2px; border-top: 2px solid #888; margin-right: 5px;"></div>
                                <span>Parent-Child</span>
                            </div>
                            <div style="display: flex; align-items: center;">
                                <div style="width: 30px; height: 0; border-top: 2px dotted #888; margin-right: 5px;"></div>
                                <span>Step-Parent</span>
                            </div>
                        `;
                        
                        document.getElementById('cyto-container').appendChild(legend);
                    });
                });
            });
        });
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
                        id: "cyto-container",
                        style: "width: 100%; height: 600px; border: 1px solid #ddd; position: relative;"
                    }
                }
            }
            // Add script tag with the cytoscape JavaScript
            script {
                dangerous_inner_html: cytoscape_js
            }
        }
    }
}
