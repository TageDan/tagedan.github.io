const drop_down_template = document.createElement("template");

drop_down_template.innerHTML = `  
  <div style="width: 100%; display: flex; justify-content: center; padding: 10px 0px 10px 0px">
    <button id="drop_down_button" style="background: none; color: #e5e7eb;"></button>
  </div>
  <div style="display: none; background: #201f35; padding: 10px 10px 10px 10px;" id="content-container">
    <slot name="content"></slot>
  </div>
`

customElements.define('drop-down',
  class extends HTMLElement {


    constructor() {
      super();



    }

    connectedCallback() {
      this.attachShadow({ mode: 'open' }).append(
        drop_down_template.content.cloneNode(true)
      );
      this.title = this.getAttribute("title");
      console.log(this.title);

      this.open = false;

      this.classList.add("w-full")

      this.shadowRoot.getElementById("drop_down_button").innerText = "V " + this.title + " V";

      this.shadowRoot.getElementById("drop_down_button").addEventListener("click", () => {
        this.open = !this.open;
        if (this.open) {
          this.shadowRoot.getElementById("drop_down_button").innerText = "^ " + this.title + " ^";
          this.shadowRoot.getElementById("content-container").style.display = "block";

        } else {
          this.shadowRoot.getElementById("drop_down_button").innerText = "V " + this.title + " V";
          this.shadowRoot.getElementById("content-container").style.display = "none";
        }
      })
    }
  }
);
