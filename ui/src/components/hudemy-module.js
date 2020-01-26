import { LitElement, html } from 'lit-element';

import '@authentic/mwc-card';
import '@authentic/mwc-list';

export class hUdemyModule extends LitElement {
  static get properties() {
    return {
      module: {
        type: Object
      }
    };
  }

  render() {
    return html`
      <mwc-card>
        <span slot="header">${this.module.title}</span>
        <mwc-list>
          ${this.module.contents.map(
            content => html`
              <mwc-list-item @click=${() => window.open(conten.url)}>
                <span slot="primary">${content.title}</span>
                <span slot="secondary">${content.description}</span>
              </mwc-list-item>
            `
          )}
        </mwc-list>

        <mwc-button
          slot="action-buttons"
          icon="add"
          label="Add content"
        ></mwc-button>
      </mwc-card>
    `;
  }
}
