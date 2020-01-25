import { LitElement, html } from "lit-element";

import "@material/mwc-top-app-bar";
import "@material/mwc-tab";
import "@material/mwc-tab-bar";

const tabs = ["enrolled-courses", "my-courses", "all-courses"];

export class hUdemyApp extends LitElement {
  firstUpdated() {
    this.activeTab = 0;
  }

  static get properties() {
    return {
      activeTab: {
        type: Number
      }
    };
  }

  render() {
    return html`
      <div class="column">
        <mwc-top-app-bar>
          <div slot="title">hUdemy</div>
        </mwc-top-app-bar>

        <mwc-tab-bar @activated=${e => (this.activeTab = e.detail.index)}>
          <mwc-tab label="Enrolled Courses"> </mwc-tab>
          <mwc-tab label="My Courses"> </mwc-tab>
          <mwc-tab label="All courses"> </mwc-tab>
        </mwc-tab-bar>

        <hudemy-courses .filter=${tabs[this.activeTab]}></hudemy-courses>
      </div>
    `;
  }
}
