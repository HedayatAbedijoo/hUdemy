import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-card';
import '@authentic/mwc-list';
import '@material/mwc-dialog';
import '@material/mwc-textfield';
import '@material/mwc-textarea';
import '@material/mwc-icon-button';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import { UPDATE_MODULE } from '../graphql/queries';

export class hUdemyModule extends LitElement {
  static get properties() {
    return {
      module: {
        type: Object
      },
      editingTitle: {
        type: Boolean
      }
    };
  }

  static get styles() {
    return [
      sharedStyles,
      css`
        .dialog-field {
          padding-top: 16px;
          padding-bottom: 16px;
        }
      `
    ];
  }

  async createContent() {
    const client = await getClient();
  }

  async updateModule() {
    this.editingTitle = false;

    const client = await getClient();
    client.mutate({
      mutation: UPDATE_MODULE,
      variables: {
        moduleId: this.module.id,
        title: this.renameModule
      }
    });
  }

  renderCreateContentDialog() {
    return html`
      <mwc-dialog id="create-content-dialog" heading="Create content">
        <div class="column" style="width: 500px;">
          <mwc-textfield
            class="dialog-field"
            label="Title"
            dialogInitialFocus
            @input=${e => (this.moduleTitle = e.target.value)}
          >
          </mwc-textfield>
          <mwc-textarea
            class="dialog-field"
            label="Description"
            @input=${e => (this.contentDescription = e.target.value)}
          >
          </mwc-textarea>
          <mwc-textfield
            class="dialog-field"
            label="URL"
            @input=${e => (this.contentUrl = e.target.value)}
          >
          </mwc-textfield>
        </div>

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

  renderTitle() {
    return html`
      <div class="row" style="align-items: center; padding-bottom: 24px;">
        ${this.editingTitle
          ? html`
              <mwc-textfield
                @input=${e => (this.renameModule = e.target.value)}
                .value=${this.module.title}
              ></mwc-textfield>
              <mwc-icon-button
                label="Save"
                icon="done"
                @click=${() => this.updateModule()}
              ></mwc-icon-button>
              <mwc-icon-button
                label="Cancel"
                icon="clear"
                @click=${() => (this.editingTitle = false)}
              ></mwc-icon-button>
            `
          : html`
              <span class="title"> ${this.module.title}</span>
              <mwc-icon-button
                label="Edit"
                icon="edit"
                @click=${() => (this.editingTitle = true)}
              ></mwc-icon-button>
            `}
      </div>
    `;
  }

  render() {
    return html`
      ${this.renderCreateContentDialog()}

      <mwc-card class="fill">
        <div style="padding: 16px;" class="column">
          ${this.renderTitle()}
          ${this.module.contents.length === 0
            ? html`
                <span class="placeholder-message">
                  There are no contents in this module
                </span>
              `
            : html`
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
              `}
        </div>
        <mwc-button
          slot="action-buttons"
          icon="add"
          label="Add content"
          @click=${() =>
            (this.shadowRoot.getElementById(
              'create-content-dialog'
            ).open = true)}
        ></mwc-button>
      </mwc-card>
    `;
  }
}
