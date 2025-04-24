use dioxus::prelude::*;

#[component]
pub fn Tree() -> Element {
    let family_tree_js = r###"
        import * as d3 from 'https://cdn.jsdelivr.net/npm/d3@7/+esm';
        import f3 from 'https://cdn.jsdelivr.net/npm/family-chart@0.2.1/+esm';
        
        // Load CSS
        const link = document.createElement('link');
        link.rel = 'stylesheet';
        link.href = 'https://cdn.jsdelivr.net/npm/family-chart@0.2.1/styles/family-chart.css';
        document.head.appendChild(link);

        // Add custom CSS for gender colors - more specific selectors to ensure they apply
        const customStyles = document.createElement('style');
        customStyles.textContent = `
            .f3 .card.male rect.card-body { 
                fill: #d1e7ff !important; 
                stroke: #a8c7f0;
            }
            .f3 .card.female rect.card-body { 
                fill: #ffe0e0 !important; 
                stroke: #f0c8c8;
            }
            .f3-control-panel { 
                position: sticky;
                top: 0;
                padding: 15px;
                background-color: #f8f9fa;
                border-bottom: 1px solid #dee2e6;
                display: flex;
                justify-content: space-between;
                align-items: center;
                z-index: 100;
                width: 100%;
            }
            .f3-tree-container {
                height: calc(100vh - 150px);
                width: 100%;
                overflow: auto;
                padding: 20px;
            }
            #FamilyChart {
                display: flex;
                flex-direction: column;
                height: 100vh;
                width: 100%;
            }
        `;
        document.head.appendChild(customStyles);

        fetch('https://donatso.github.io/family-chart-doc/data/wikidata-popular.json')
          .then(res => res.json())
          .then(data => create(data))
          .catch(err => console.error(err))

        function create(data) {
          // Get the main container
          const chartDiv = document.querySelector("div#FamilyChart");
          chartDiv.innerHTML = ''; // Clear any existing content
          
          // Create control panel for search and button
          const controlPanel = document.createElement('div');
          controlPanel.className = 'f3-control-panel';
          chartDiv.appendChild(controlPanel);
          
          // Create tree container that will take the remaining space
          const treeContainer = document.createElement('div');
          treeContainer.className = 'f3-tree-container';
          treeContainer.id = 'FamilyTreeContainer';
          chartDiv.appendChild(treeContainer);
          
          const store = f3.createStore({
            data,
            node_separation: 250,
            level_separation: 150
          });
          
          const svg = f3.createSvg(treeContainer);
          
          // Improved card styling with gender classes
          const Card = f3.elements.Card({
            store,
            svg,
            card_dim: {w:220,h:70,text_x:75,text_y:15,img_w:60,img_h:60,img_x:5,img_y:5},
            card_display: [d => `${d.data["label"]}`, d => d.data["desc"]],
            mini_tree: true,
            link_break: false,
            on_card_click: d => {
              updateTreeWithNewMainPerson(d.id, true);
            },
            card_g_class: d => {
              // Add gender class based on data
              return d.data.gender === 'male' ? 'male' : 
                     d.data.gender === 'female' ? 'female' : '';
            }
          });

          // Set up view update handler
          store.setOnUpdate(props => {
            f3.view(store.getTree(), svg, Card, props || {});
            
            // Force gender classes to apply after rendering
            setTimeout(() => {
              document.querySelectorAll('.card').forEach(card => {
                if (card.__data__ && card.__data__.data && card.__data__.data.gender) {
                  const gender = card.__data__.data.gender;
                  if (gender === 'male') card.classList.add('male');
                  if (gender === 'female') card.classList.add('female');
                }
              });
            }, 100);
          });
          
          // Initialize with Charles III
          store.updateMainId('Q43274');
          store.updateTree({initial: true});

          // Function to update the tree with a new main person
          function updateTreeWithNewMainPerson(person_id, animation_initial = true) {
            store.updateMainId(person_id);
            store.updateTree({initial: animation_initial});
          }

          // Create search container
          const searchContainer = document.createElement('div');
          searchContainer.style.width = '200px';
          controlPanel.appendChild(searchContainer);
          
          // Create search input
          const searchInput = document.createElement('input');
          searchInput.type = 'text';
          searchInput.placeholder = 'Search';
          searchInput.className = 'form-control';
          searchContainer.appendChild(searchInput);
          
          // Create dropdown for search results
          const dropdown = document.createElement('div');
          dropdown.style.position = 'absolute';
          dropdown.style.width = '200px';
          dropdown.style.maxHeight = '300px';
          dropdown.style.overflowY = 'auto';
          dropdown.style.backgroundColor = 'white';
          dropdown.style.border = '1px solid #dee2e6';
          dropdown.style.borderRadius = '0 0 4px 4px';
          dropdown.style.display = 'none';
          dropdown.style.zIndex = '1000';
          searchContainer.appendChild(dropdown);
          
          // Create random family button
          const randomButton = document.createElement('button');
          randomButton.textContent = 'View Random Family';
          randomButton.className = 'btn btn-primary';
          controlPanel.appendChild(randomButton);
          
          randomButton.addEventListener('click', () => {
            const random_person = data[Math.floor(Math.random() * data.length)];
            const person_id = random_person["id"];
            updateTreeWithNewMainPerson(person_id, false);
          });
          
          // Setup search functionality
          const all_select_options = [];
          data.forEach(d => {
            if (all_select_options.find(d0 => d0.value === d["id"])) return;
            all_select_options.push({label: `${d.data["label"]}`, value: d["id"]});
          });
          
          searchInput.addEventListener('focus', activateDropdown);
          searchInput.addEventListener('input', activateDropdown);
          
          document.addEventListener('click', (e) => {
            if (!searchContainer.contains(e.target)) {
              dropdown.style.display = 'none';
            }
          });
          
          function activateDropdown() {
            const search_value = searchInput.value.toLowerCase();
            const filtered_options = all_select_options.filter(d => 
              d.label.toLowerCase().includes(search_value)
            );
            
            updateDropdown(filtered_options);
            
            if (filtered_options.length > 0 && search_value) {
              dropdown.style.display = 'block';
            } else {
              dropdown.style.display = 'none';
            }
          }
          
          function updateDropdown(filtered_options) {
            dropdown.innerHTML = '';
            
            filtered_options.slice(0, 10).forEach(option => {
              const item = document.createElement('div');
              item.textContent = option.label;
              item.style.padding = '8px 12px';
              item.style.cursor = 'pointer';
              item.style.borderBottom = '1px solid #dee2e6';
              
              item.addEventListener('mouseenter', () => {
                item.style.backgroundColor = '#f8f9fa';
              });
              
              item.addEventListener('mouseleave', () => {
                item.style.backgroundColor = 'white';
              });
              
              item.addEventListener('click', () => {
                updateTreeWithNewMainPerson(option.value, true);
                dropdown.style.display = 'none';
                searchInput.value = option.label;
              });
              
              dropdown.appendChild(item);
            });
          }
        }
    "###;

    rsx! {
        div {
            class: "container-fluid p-0 h-100",
            // Tree Container - full height and width
            div {
                id: "FamilyChart",
                class: "f3",
                style: "width: 100%; height: 100vh; overflow: hidden; background-color: #f0f2f5;"
            }
            // Add script tag with the family tree JavaScript
            script {
                r#"type"#: "module",
                dangerous_inner_html: family_tree_js
            }
        }
    }
}
