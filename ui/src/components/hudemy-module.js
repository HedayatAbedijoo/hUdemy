import { LitElement, html, css } from 'lit-element';

import '@authentic/mwc-card';
import '@authentic/mwc-list';
import '@material/mwc-dialog';
import '@material/mwc-textfield';
import '@material/mwc-textarea';
import '@material/mwc-icon-button';

import { sharedStyles } from '../shared-styles';
import { getClient } from '../graphql';
import {
  UPDATE_MODULE,
  CREATE_CONTENT,
  DELETE_MODULE
} from '../graphql/queries';

export class hUdemyModule extends LitElement {
  static get properties() {
    return {
      module: {
        type: Object
      },
      editable: {
        type: Boolean
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

        .content-title {
          font-size: 18px;
        }

        hr {
          padding: 0;
          margin: 0;
        }

        .content-item {
          padding: 4px;
        }

        .content-list {
        }
      `
    ];
  }

  async createContent() {
    const client = await getClient();

    client.mutate({
      mutation: CREATE_CONTENT,
      variables: {
        moduleId: this.module.id,
        content: {
          name: this.contentName,
          url: this.contentUrl,
          description: this.contentDescription
        }
      }
    });
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

  async deleteModule() {
    const client = await getClient();
    client.mutate({
      mutation: DELETE_MODULE,
      variables: {
        moduleId: this.module.id
      }
    });
  }

  renderCreateContentDialog() {
    return html`
      <mwc-dialog id="create-content-dialog" heading="Add content">
        <div class="column" style="width: 500px; margin-top: 16px;">
          <mwc-textfield
            outlined
            class="dialog-field"
            label="Name"
            dialogInitialFocus
            @input=${e => (this.contentName = e.target.value)}
          >
          </mwc-textfield>
          <mwc-textarea
            class="dialog-field"
            label="Description"
            @input=${e => (this.contentDescription = e.target.value)}
          >
          </mwc-textarea>
          <mwc-textfield
            outlined
            class="dialog-field"
            label="URL"
            @input=${e => (this.contentUrl = e.target.value)}
          >
          </mwc-textfield>
        </div>

        <mwc-button
          slot="primaryAction"
          dialogAction="create"
          @click=${() => this.createContent()}
        >
          Create
        </mwc-button>
        <mwc-button slot="secondaryAction" dialogAction="cancel">
          Cancel
        </mwc-button>
      </mwc-dialog>
    `;
  }

  renderHeader() {
    return html`
      <div class="row">
        <div style="flex: 1;">${this.renderTitle()}</div>
        ${this.renderToolbar()}
      </div>
    `;
  }

  renderTitle() {
    return html`
      <div class="row" style="align-items: center; padding-bottom: 24px;">
        ${this.editable && this.editingTitle
          ? html`
              <mwc-textfield
                outlined
                @input=${e => (this.renameModule = e.target.value)}
                .value=${this.module.title}
              ></mwc-textfield>
            `
          : html`
              <span class="title"> ${this.module.title}</span>
            `}
      </div>
    `;
  }

  renderToolbar() {
    if (!this.editable) return html``;

    if (this.editingTitle)
      return html`
        <div class="row">
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
        </div>
      `;

    return html`
      <div class="row">
        <mwc-icon-button
          label="Edit"
          icon="edit"
          @click=${() => (this.editingTitle = true)}
        ></mwc-icon-button>
        <mwc-icon-button
          slot="action-buttons"
          icon="add"
          label="Add content"
          @click=${() =>
            (this.shadowRoot.getElementById(
              'create-content-dialog'
            ).open = true)}
        ></mwc-icon-button>
        <mwc-icon-button
          slot="action-buttons"
          icon="delete"
          label="Delete module"
          @click=${() => this.deleteModule()}
        ></mwc-icon-button>
      </div>
    `;
  }

  render() {
    return html`
      ${this.renderCreateContentDialog()}

      <mwc-card class="fill">
        <div style="padding: 16px;" class="column">
          ${this.renderHeader()}
          ${this.module.contents.length === 0
            ? html`
                <span class="placeholder-message">
                  There are no contents in this module
                </span>
              `
            : html`
                <span style="padding-bottom: 8px;">Contents</span>
                <mwc-list>
                  <div class="content-list">
                    ${this.module.contents.map(
                      (content, index) => html`
                        <mwc-list-item
                          @click=${() => window.open(content.url)}
                          class="content-item"
                        >
                          <div class="column">
                            <span class="content-title">${content.name}</span>
                            <span class="fading">${content.description}</span>
                          </div>
                        </mwc-list-item>
                        ${index !== this.module.contents.length - 1
                          ? html`
                              <hr style="opacity: 0.6" />
                            `
                          : html``}
                      `
                    )}
                  </div>
                </mwc-list>
              `}
        </div>
      </mwc-card>
    `;
  }
}
