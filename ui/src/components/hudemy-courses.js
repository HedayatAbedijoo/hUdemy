import { LitElement, html } from "lit-element";

import "@material/mwc-drawer";

export class hUdemyCourses extends LitElement {
  static get properties() {
    return {
      myCourses: {
        type: Boolean
      }
    };
  }

  render() {
    return html`
      <mwc-drawer>
        

      </mwc-drawer>
    `;
  }
}
