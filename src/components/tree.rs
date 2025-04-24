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

        // Load required libraries for dTree
        loadScripts([
            'https://cdnjs.cloudflare.com/ajax/libs/d3/7.8.5/d3.min.js',
            'https://cdnjs.cloudflare.com/ajax/libs/lodash.js/4.17.21/lodash.min.js',
            'https://cdn.jsdelivr.net/npm/d3-dtree@2.4.1/dist/dTree.min.js'
        ], function() {
            try {
                // Sample family data in dTree format
                const data = [
                    {
                        name: "John Doe",
                        class: "male",
                        textClass: "emphasis",
                        marriage: {
                            spouse: {
                                name: "Jane Smith",
                                class: "female"
                            },
                            children: [
                                {
                                    name: "Alice Doe",
                                    class: "female"
                                },
                                {
                                    name: "Bob Doe",
                                    class: "male",
                                    marriage: {
                                        spouse: {
                                            name: "Carol Johnson",
                                            class: "female"
                                        },
                                        children: [
                                            {
                                                name: "David Doe",
                                                class: "male"
                                            }
                                        ]
                                    }
                                }
                            ]
                        }
                    }
                ];

                // Configure the tree
                const config = {
                    data: data,
                    target: "#family-chart-container",
                    debug: true,
                    height: 550,
                    width: document.getElementById('family-chart-container').offsetWidth,
                    callbacks: {
                        nodeClick: function(name, extra, id) {
                            console.log("Node clicked:", name, extra, id);
                        }
                    },
                    nodeWidth: 120,
                    nodeHeight: 70,
                    styles: {
                        node: 'node',
                        linage: 'linage',
                        marriage: 'marriage',
                        text: 'nodeText'
                    }
                };

                // Draw the tree using the new initialization format
                dTree.init(config);
                
                console.log("dTree initialization complete");
                
                // Check if SVG was created
                const svg = document.querySelector('#family-chart-container svg');
                if (!svg) {
                    console.error("No SVG created by dTree");
                    
                    // Force create SVG and draw a simple tree as fallback
                    const container = document.getElementById('family-chart-container');
                    const width = container.offsetWidth;
                    const height = 550;
                    
                    const svg = d3.select('#family-chart-container')
                        .append('svg')
                        .attr('width', width)
                        .attr('height', height);
                        
                    // Create a simple tree layout
                    const treeData = d3.hierarchy({
                        name: "John Doe",
                        children: [
                            { name: "Alice Doe" },
                            { 
                                name: "Bob Doe",
                                children: [{ name: "David Doe" }]
                            }
                        ]
                    });
                    
                    const treeLayout = d3.tree().size([width - 100, height - 100]);
                    treeLayout(treeData);
                    
                    // Add links
                    svg.append('g')
                        .attr('transform', 'translate(50,50)')
                        .selectAll('path')
                        .data(treeData.links())
                        .enter()
                        .append('path')
                        .attr('d', d3.linkVertical()
                            .x(d => d.x)
                            .y(d => d.y))
                        .attr('fill', 'none')
                        .attr('stroke', '#333')
                        .attr('stroke-width', 2);
                    
                    // Add nodes
                    svg.append('g')
                        .attr('transform', 'translate(50,50)')
                        .selectAll('circle')
                        .data(treeData.descendants())
                        .enter()
                        .append('circle')
                        .attr('cx', d => d.x)
                        .attr('cy', d => d.y)
                        .attr('r', 20)
                        .attr('fill', '#a8d1f0')
                        .attr('stroke', '#333')
                        .attr('stroke-width', 2);
                    
                    // Add labels
                    svg.append('g')
                        .attr('transform', 'translate(50,50)')
                        .selectAll('text')
                        .data(treeData.descendants())
                        .enter()
                        .append('text')
                        .attr('x', d => d.x)
                        .attr('y', d => d.y + 35)
                        .attr('text-anchor', 'middle')
                        .text(d => d.data.name)
                        .attr('font-family', 'Arial')
                        .attr('font-size', '12px');
                        
                    console.log("Fallback tree created");
                }

                // Add custom CSS for styling
                const style = document.createElement('style');
                style.textContent = `
                    .node.male { 
                        fill: #a8d1f0; 
                        stroke: #333;
                        stroke-width: 2px;
                    }
                    .node.female { 
                        fill: #f0c1d8; 
                        stroke: #333;
                        stroke-width: 2px;
                    }
                    .nodeText {
                        font-family: Arial, sans-serif;
                        font-size: 14px;
                        fill: #333;
                    }
                    .emphasis {
                        font-weight: bold;
                    }
                    .linage {
                        fill: none;
                        stroke: #333;
                        stroke-width: 2px;
                    }
                    .marriage {
                        fill: none;
                        stroke: #333;
                        stroke-width: 2px;
                    }
                `;
                document.head.appendChild(style);

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
                
                document.getElementById('family-chart-container').appendChild(legend);
            } catch (error) {
                console.error('Error initializing family tree:', error);
                const container = document.getElementById('family-chart-container');
                if (container) {
                    container.innerHTML = '<div class="alert alert-danger">Error initializing family tree: ' + error.message + '</div>';
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
