import { LitElement, html } from 'lit-element';

import '@authentic/mwc-card';
import '@authentic/mwc-list';
import '@authentic/mwc-circular-progress';

import { router } from '../router';
import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import { GET_COURSES } from '../graphql/queries';

export class LeapCoursesList extends LitElement {
  static get properties() {
    return {
      filter: {
        type: String
      },
      courses: {
        type: Array
      }
    };
  }

  static get styles() {
    return sharedStyles;
  }

  async firstUpdated() {
    this.loadCourses();
  }

  async loadCourses() {
    this.courses = undefined;

    const client = await getClient();
    const result = await client.query({
      query: GET_COURSES,
      variables: {
        filter: this.filter || 'all'
      }
    });

    this.courses = result.data.courses;

    if (this.courses.length > 0) {
      this.selectedCourseId = this.courses[0].id;
    }
  }

  updated(changedValues) {
    super.updated(changedValues);

    if (changedValues.get('filter')) {
      this.loadCourses();
    }
  }

  renderEmptyPlaceholder() {
    return html`
      <div class="fill center-content">
        <h3 class="fading">There are no courses in this category</h3>
      </div>
    `;
  }

  render() {
    if (!this.courses)
      return html`
        <div class="fill center-content">
          <mwc-circular-progress></mwc-circular-progress>
        </div>
      `;

    if (this.courses.length === 0) return this.renderEmptyPlaceholder();

    return html`
      <mwc-list style="width: 500px;">
        ${this.courses.map(
          course => html`
            <mwc-list-item
              @click=${() => router.navigate(`course/${course.id}`)}
            >
              <span>${course.title}</span>
            </mwc-list-item>
          `
        )}
      </mwc-list>
    `;
  }
}
