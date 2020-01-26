import { LitElement, html } from 'lit-element';
import { router } from '../router';

export class LeapApp extends LitElement {
  static get properties() {
    return {
      appContent: {
        type: Object
      }
    };
  }

  constructor() {
    super();
    router
      .on(
        'home',
        () =>
          (this.appContent = html`
            <leap-dashboard></leap-dashboard>
          `)
      )
      .on(
        'course/:id',
        params =>
          (this.appContent = html`
            <leap-course-detail .courseId=${params.id}></leap-course-detail>
          `)
      )
      .on('/', () => router.navigate('/home'))
      .resolve();
  }

  render() {
    return this.appContent;
  }
}
