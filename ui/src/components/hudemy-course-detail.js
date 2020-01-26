import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-circular-progress';
import '@material/mwc-fab';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import { GET_COURSE_INFO, CREATE_MODULE } from '../graphql/queries';

export class hUdemyCourseDetail extends LitElement {
  static get properties() {
    return {
      courseId: {
        type: String
      },
      course: {
        type: Object
      }
    };
  }

  static get styles() {
    return [
      sharedStyles,
      css`
        .fab {
          position: absolute;
          bottom: 16px;
          right: 16px;
        }
      `
    ];
  }

  async loadCourse() {
    this.course = undefined;

    const client = await getClient();

    const result = await client.query({
      query: GET_COURSE_INFO,
      variables: {
        courseId: this.courseId
      }
    });

    this.course = result.data.course;
  }

  firstUpdated() {
    this.loadCourse();
  }

  async createModule() {
    const client = await getClient();

    const result = await client.mutate({
      mutation: CREATE_MODULE,
      variables: {
        courseId: this.courseId,
        title: this.moduleTitle
      }
    });

  }

  renderCreateModuleDialog() {
    return html`
      <mwc-dialog id="create-module-dialog" heading="Create module">
        <mwc-textfield
          label="Title"
          dialogInitialFocus
          @input=${e => (this.moduleTitle = e.target.value)}
        >
        </mwc-textfield>

        <mwc-button
          slot="primaryAction"
          dialogAction="create"
          @click=${() => this.createModule()}
        >
          Create
        </mwc-button>
        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
      </mwc-dialog>
    `;
  }

  render() {
    if (!this.course)
      return html`
        <mwc-circular-progress></mwc-circular-progress>
      `;

    return html`
      ${this.renderCreateModuleDialog()}
      <div class="column fill center-content" style="position: relative;">
        ${this.course.modules.length === 0
          ? html`
              <h3 class="fading">There are no modules in this course</h3>
            `
          : this.course.modules.map(
              module =>
                html`
                  <hudemy-module .module=${module}></hudemy-module>
                `
            )}

        <mwc-fab
          class="fab"
          extended
          icon="add"
          label="Add module"
          @click=${() =>
            (this.shadowRoot.getElementById(
              'create-module-dialog'
            ).open = true)}
        ></mwc-fab>
      </div>
    `;
  }
}
