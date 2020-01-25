import { LitElement, html } from "lit-element";

import "@material/mwc-drawer";
import "@authentic/mwc-list";
import "@authentic/mwc-circular-progress";

import { getClient } from "../graphql";
import { GET_ALL_COURSES } from "../graphql/queries";

export class hUdemyCoursesDrawer extends LitElement {
  static get properties() {
    return {
      filter: {
        type: String
      },
      courses: {
        type: Array
      },
      selectedCourse: {
        type: String
      }
    };
  }

  async firstUpdated() {
    this.client = await getClient();
    this.loadCourses();
  }

  async loadCourses() {
    this.courses = undefined;
    const result = await this.client.query({
      query: GET_ALL_COURSES
    });

    this.courses = result.data.allCourses;
  }

  render() {
    if (!this.courses)
      return html`
        <mwc-circular-progress></mwc-circular-progress>
      `;

    return html`
      <mwc-drawer>
        <mwc-list>
          ${this.courses.map(
            course => html`
              <mwc-list-item class="row">
                <span style="flex: 1;">${course.title}</span>
                <span style="opacity: 0.5;">By ${course.teacher_address}</span>
              </mwc-list-item>
            `
          )}
        </mwc-list>

        <div slot="appContent">
          ${this.selectedCourseId
            ? html`
                <hudemy-course-detail .courseId=${this.selectedCourseId}>
                </hudemy-course-detail>
              `
            : html`
                <h3>No course selected</h3>
              `}
        </div>
      </mwc-drawer>
    `;
  }
}
