import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-circular-progress';
import '@material/mwc-button';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import {
  GET_COURSE_INFO,
  CREATE_MODULE,
  ENROL_IN_COURSE,
  DELETE_COURSE
} from '../graphql/queries';

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
    this.myAddress = result.data.myAddress;
  }

  firstUpdated() {
    this.loadCourse();
  }

  updated(changedValues) {
    super.updated(changedValues);

    if (changedValues.get('courseId')) {
      this.loadCourse();
    }
  }

  async createModule() {
    const client = await getClient();

    const result = await client.mutate({
      mutation: CREATE_MODULE,
      variables: {
        courseId: this.courseId,
        title: this.moduleTitle
      },
      refetchQueries: [
        {
          query: GET_COURSE_INFO,
          variables: {
            courseId: this.courseId
          }
        }
      ]
    });

    this.loadCourse();
  }

  renderCreateModuleDialog() {
    return html`
      <mwc-dialog id="create-module-dialog" heading="Create module">
        <mwc-textfield
          style="margin-top: 16px;"
          outlined
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

  renderPlacholder(message) {
    return html`
      <div class="fill center-content">
        <span class="placeholder-message">
          ${message}
        </span>
      </div>
    `;
  }

  renderModules() {
    if (this.course.modules.length === 0)
      return this.renderPlacholder('There are no modules in this course');

    return html`
      <div class="column">
        ${this.course.modules.map(
          module =>
            html`
              <hudemy-module
                .module=${module}
                .editable=${this.userIsTeacher()}
              ></hudemy-module>
            `
        )}
      </div>
    `;
  }

  userIsTeacher() {
    return this.myAddress === this.course.teacher_address;
  }

  async enrolInCourse() {
    const client = await getClient();

    client.mutate({
      mutation: ENROL_IN_COURSE,
      variables: {
        courseId: this.courseId
      }
    });
  }

  async deleteCourse() {
    const client = await getClient();

    client.mutate({
      mutation: DELETE_COURSE,
      variables: {
        courseId: this.courseId
      }
    });
  }

  renderCourseInfo() {
    return html`
      <div class="row center-content" style="padding-bottom: 24px;">
        <div class="column fill">
          <h2>${this.course.title}</h2>
          <span>Taught by ${this.course.teacher_address}</span>
        </div>

        ${this.userIsTeacher()
          ? html`
              <div class="column">
                <mwc-button
                  icon="add"
                  label="Add module"
                  raised
                  style="padding-bottom: 8px;"
                  @click=${() =>
                    (this.shadowRoot.getElementById(
                      'create-module-dialog'
                    ).open = true)}
                ></mwc-button>

                <mwc-button
                  icon="delete"
                  label="Delete course"
                  outlined
                  class="danger"
                  @click=${() => this.deleteCourse()}
                ></mwc-button>
              </div>
            `
          : html`
              <mwc-button
                icon="school"
                label="Enrol in this course"
                outlined
                @click=${() => this.enrolInCourse()}
              ></mwc-button>
            `}
      </div>
    `;
  }

  renderStudentsList() {
    if (this.course.students.length === 0)
      return this.renderPlacholder(
        'There are no students enrolled in this course'
      );

    return html`
      <mwc-list>
        ${this.course.students.map(
          student => html`
            <span>
              ${student}
            </span>
          `
        )}
      </mwc-list>
    `;
  }

  render() {
    if (!this.course)
      return html`
        <div class="column fill center-content" style="position: relative;">
          <mwc-circular-progress></mwc-circular-progress>
        </div>
      `;

    return html`
      ${this.renderCreateModuleDialog()}

      <div class="column" style="position: relative; padding: 16px;">
        ${this.renderCourseInfo()}

        <div class="row">
          <div class="column" style="flex: 1; padding-right: 24px;">
            <h3>Modules</h3>
            ${this.renderModules()}
          </div>

          <div class="column">
            <h3>Students</h3>
            ${this.renderStudentsList()}
          </div>
        </div>
      </div>
    `;
  }
}
