const terminal = document.getElementById("terminal");
const terminal_inputs = document.getElementsByClassName("terminal-input");

// Creates a new `section` for every `Enter` keypress
terminal.addEventListener("keypress", (event) => {
  // Check if the key pressed is 'Enter'
  if (event.key === "Enter") {
    // Create and set up the new elements
    const section = document.createElement("section");

    const firstDiv = document.createElement("div");
    firstDiv.innerHTML = `
                <span class="name">
                <span class="user">rezabet</span>@<span class="computer"
                    >elementary-os</span
                ></span
                >:<span class="path">/home/rezabet/friends</span>$
            `;

    const secondDiv = document.createElement("div");
    const terminal_input = document.createElement("input");
    terminal_input.setAttribute("id", "terminal-input");
    terminal_input.setAttribute("class", "terminal-input");
    terminal_input.setAttribute("type", "text");
    terminal_input.setAttribute("autocomplete", "off");
    secondDiv.appendChild(terminal_input);

    // Append the divs to the section
    section.appendChild(firstDiv);
    section.appendChild(secondDiv);

    // Append the section to `terminal`
    terminal.appendChild(section);

    // Sets focus to the newly created `terminal-input`
    terminal_input.focus();
  }
});
