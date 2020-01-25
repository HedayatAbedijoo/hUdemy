import { LitElement, html } from "lit-element";

export class hUdemyCourseDetail extends LitElement {
  static get properties() {
    return {
      courseId: {
        type: String
      }
    };
  }

  render() {
    return html`
      <mwc-drawer> </mwc-drawer>
    `;
  }
}
