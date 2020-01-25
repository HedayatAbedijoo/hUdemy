import { LitElement, html } from 'lit-element';

import '@authentic/mwc-card';

export class hUdemyModule extends LitElement {
  static get properties() {
    return {
      myCourses: {
        type: Boolean
      }
    };
  }

  render() {
    return html`
      <mwc-card> </mwc-card>
    `;
  }
}
