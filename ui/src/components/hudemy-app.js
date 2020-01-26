import { LitElement, html } from 'lit-element';

import '@material/mwc-top-app-bar';
import '@material/mwc-tab';
import '@material/mwc-tab-bar';
import '@material/mwc-dialog';
import '@material/mwc-textfield';
import '@material/mwc-button';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import { CREATE_COURSE } from '../graphql/queries';

const tabs = ['enrolled-courses', 'my-courses', 'all-courses'];

export class hUdemyApp extends LitElement {
  static get styles() {
    return sharedStyles;
  }

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

  renderCreateCourseDialog() {
    return html`
      <mwc-dialog id="create-course-dialog" heading="Create course">
        <mwc-textfield
          style="margin-top: 16px;"
          outlined
          label="Title"
          dialogInitialFocus
          @input=${e => (this.courseTitle = e.target.value)}
        >
        </mwc-textfield>

        <mwc-button
          slot="primaryAction"
          dialogAction="create"
          @click=${() => this.createCourse()}
        >
          Create
        </mwc-button>
        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
      </mwc-dialog>
    `;
  }

  async createCourse() {
    const client = await getClient();

    await client.mutate({
      mutation: CREATE_COURSE,
      variables: {
        title: this.courseTitle
      }
    });
  }

  render() {
    return html`
      ${this.renderCreateCourseDialog()}
      <div class="column fill">
        <mwc-top-app-bar>
          <div slot="title">hUdemy</div>

          <mwc-button
            slot="actionItems"
            label="Create course"
            icon="add"
            @click=${() =>
              (this.shadowRoot.getElementById(
                'create-course-dialog'
              ).open = true)}
          ></mwc-button>
        </mwc-top-app-bar>

        <mwc-tab-bar
          @MDCTabBar:activated=${e => (this.activeTab = e.detail.index)}
        >
          <mwc-tab label="Enrolled Courses"> </mwc-tab>
          <mwc-tab label="My Courses"> </mwc-tab>
          <mwc-tab label="All courses"> </mwc-tab>
        </mwc-tab-bar>

        <hudemy-courses-drawer
          class="fill"
          .filter=${tabs[this.activeTab]}
        ></hudemy-courses-drawer>
      </div>
    `;
  }
}
