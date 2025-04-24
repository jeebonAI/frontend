use dioxus::prelude::*;

#[component]
pub fn Tree() -> Element {
    let family_tree_js = r###"
        function loadScripts(urls, callback) {
            let loaded = 0;
            urls.forEach(url => {
                const script = document.createElement('script');
                script.type = 'text/javascript';
                script.src = url;
                script.onload = () => {
                    loaded++;
                    if (loaded === urls.length) {
                        callback();
                    }
                };
                document.head.appendChild(script);
            });
        }

        // Load required libraries
        loadScripts([
            'https://d3js.org/d3.v7.min.js',
            'https://unpkg.com/family-chart@latest/dist/family-chart.min.js'
        ], function() {
            try {
                const container = document.getElementById('family-chart-container');
                
                // Sample family data
                const data = [
                    { id: 1, gender: 'male', parents: [], children: [3, 4], spouses: [2], name: 'John Doe' },
                    { id: 2, gender: 'female', parents: [], children: [3, 4], spouses: [1], name: 'Jane Smith' },
                    { id: 3, gender: 'female', parents: [1, 2], children: [], spouses: [], name: 'Alice Doe' },
                    { id: 4, gender: 'male', parents: [1, 2], children: [], spouses: [], name: 'Bob Doe' }
                ];
                
                // Create chart using the correct constructor and initialization pattern
                // Based on examples from https://github.com/donatso/family-chart/tree/master/examples
                const chart = new FamilyChart({
                    container_id: 'family-chart-container',
                    data: data,
                    svgHeight: 600,
                    svgWidth: container.clientWidth,
                    nodeWidth: 120,
                    nodeHeight: 70,
                    nodePaddingX: 10,
                    nodePaddingY: 10,
                    nodeBorderRadius: 5,
                    backgroundColor: '#fff',
                    connectionsLineStyle: 'straight',
                    orientation: 'vertical',
                    debug: false,
                    onNodeClick: (d) => {
                        console.log('Node clicked:', d);
                    },
                    nodeRenderer: function(d, i, nodes) {
                        const node = document.createElementNS('http://www.w3.org/2000/svg', 'g');
                        
                        // Rectangle
                        const rect = document.createElementNS('http://www.w3.org/2000/svg', 'rect');
                        rect.setAttribute('width', '120');
                        rect.setAttribute('height', '70');
                        rect.setAttribute('rx', '5');
                        rect.setAttribute('ry', '5');
                        rect.setAttribute('fill', d.gender === 'male' ? '#a8d1f0' : '#f0c1d8');
                        rect.setAttribute('stroke', '#333');
                        rect.setAttribute('stroke-width', '2');
                        
                        // Text
                        const text = document.createElementNS('http://www.w3.org/2000/svg', 'text');
                        text.setAttribute('x', '60');
                        text.setAttribute('y', '40');
                        text.setAttribute('text-anchor', 'middle');
                        text.setAttribute('font-family', 'Arial, sans-serif');
                        text.setAttribute('font-size', '14px');
                        text.setAttribute('fill', '#333');
                        text.textContent = d.name;
                        
                        node.appendChild(rect);
                        node.appendChild(text);
                        
                        return node;
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
                        <div style="width: 15px; height: 15px; background-color: #a8d1f0; margin-right: 5px;"></div>
                        <span>Male</span>
                    </div>
                    <div style="display: flex; align-items: center; margin-bottom: 3px;">
                        <div style="width: 15px; height: 15px; background-color: #f0c1d8; margin-right: 5px;"></div>
                        <span>Female</span>
                    </div>
                `;
                
                container.appendChild(legend);
            } catch (error) {
                console.error('Error initializing family chart:', error);
                const container = document.getElementById('family-chart-container');
                if (container) {
                    container.innerHTML = '<div class="alert alert-danger">Error initializing family chart: ' + error.message + '</div>';
                }
            }
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
                        id: "family-chart-container",
                        style: "width: 100%; height: 600px; border: 1px solid #ddd; position: relative;"
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
