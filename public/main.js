const drop_down_template = document.createElement("template");

drop_down_template.innerHTML = `  
  <div style="width: 100%; display: flex; justify-content: center; padding: 10px 0px 10px 0px">
    <button id="drop_down_button" style="background: none; color: #e5e7eb; cursor: pointer; border-style: solid; border-width: 2px; border-color: white;"></button>
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
      this.title_text = this.getAttribute("caption");
      console.log(this.title);

      this.open = false;

      this.classList.add("w-full")

      this.shadowRoot.getElementById("drop_down_button").innerText = "V " + this.title_text + " V";

      this.shadowRoot.getElementById("drop_down_button").addEventListener("click", () => {
        this.open = !this.open;
        if (this.open) {
          this.shadowRoot.getElementById("drop_down_button").innerText = "^ " + this.title_text + " ^";
          this.shadowRoot.getElementById("content-container").style.display = "block";

        } else {
          this.shadowRoot.getElementById("drop_down_button").innerText = "V " + this.title_text + " V";
          this.shadowRoot.getElementById("content-container").style.display = "none";
        }
      })
    }
  }
);

const info_template = document.createElement("template");

info_template.innerHTML = `
  <span id = "container" style="position: static;">
  <span id = "word" style="color: #aaffaa; cursor: help">
  <slot name="word"></slot>
  </span>
  <div id="info" style ="display: none; position: absolute; transform: translate(-50%,0); width: 33vw; border: #e5e7eb; border-width: 2; border-style: solid; padding: 3px 3px 3px 3px; border-radius: 2px; background: #45474b;">
  <slot name = "info"></slot>
  </div>
  </span>
`

customElements.define('info-card',
  class extends HTMLElement {
    constructor() {
      super();
    }

    connectedCallback() {
      this.attachShadow({ mode: 'open' }).append(
        info_template.content.cloneNode(true)
      );
      this.open = false;
      this.container = this.shadowRoot.getElementById("container");
      this.info_card = this.shadowRoot.getElementById("info");
      this.word = this.shadowRoot.getElementById("word");
      this.show = () => {
        this.info_card.style.display = "block";
        let rect1 = this.word.getBoundingClientRect();
        let x = rect1.left + window.scrollX + rect1.width / 2;
        let y = rect1.bottom + window.scrollY;

        this.info_card.style.left = x + "px";
        this.info_card.style.top = y + "px";

        let rect2 = this.info_card.getBoundingClientRect();

        if (rect2.right >= window.innerWidth) {
          console.log(rect2.right, window.innerWidth);
          this.info_card.style.left = (x - rect2.right + window.innerWidth - 5) + "px";
        }

        if (rect2.left <= 0) {
          this.info_card.style.left = (x - rect2.left + 5) + "px";
        }





      }
      this.info_card.addEventListener("click", (e) => { e.stopPropagation() });
      this.container.addEventListener("mouseover", () => { this.show(); console.log("over") });
      this.container.addEventListener("mouseleave", () => {
        if (!this.open) {
          this.info_card.style.display = "none";
        }
      });
      this.word.addEventListener("click", (e) => {
        e.stopPropagation();
        this.open = !this.open;
        if (this.open) {
          this.show();
        } else {
          this.info_card.style.display = "none";
        }
      });
      document.addEventListener("click", () => {
        this.open = false;
        this.info_card.style.display = "none";

      });
    }
  }
);
