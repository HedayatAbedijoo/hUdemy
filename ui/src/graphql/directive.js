import { SchemaDirectiveVisitor } from "graphql-tools";

import { INSTANCE_NAME, ZOME_NAME } from "../config";
import { parseEntry } from "../utils";

export class LoadEntityDirective extends SchemaDirectiveVisitor {
  visitFieldDefinition(field, detail) {
    let defaultResolver = field.resolve;

    field.resolve = async (parent, args, { callZome }, info) => {
      const entryId = await defaultResolver(parent, args, context, info);

      if (typeof entryId === "string") return this.loadEntry(entryId, callZome);
      else return entryId.map(id => this.loadEntry(id, callZome));
    };
  }

  async loadEntry(entryId, callZome) {
    const entryResult = await callZome.call(
      INSTANCE_NAME,
      ZOME_NAME,
      "get_entry",
      {
        address: entryId
      }
    );

    const entry = parseEntry(entryResult);

    return { id: entryId, ...entry };
  }
}
