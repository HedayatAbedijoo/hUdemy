import { LitElement, html } from "lit-element";

import "@material/mwc-top-app-bar";
import "@material/mwc-tab";
import "@material/mwc-tab-bar";

export class hUdemyApp extends LitElement {
  render() {
    return html`
      <div class="column">
        <mwc-top-app-bar>
          <div slot="title">hUdemy</div>
        </mwc-top-app-bar>

        <mwc-tab-bar>
          <mwc-tab label="My Courses"> </mwc-tab>
          <mwc-tab label="All courses"> </mwc-tab>
        </mwc-tab-bar>
      </div>
    `;
  }
}
