import { SchemaDirectiveVisitor } from 'graphql-tools';

import { INSTANCE_NAME, ZOME_NAME } from '../config';
import { parseEntry } from '../utils';

export class LoadEntityDirective extends SchemaDirectiveVisitor {
  visitFieldDefinition(field, detail) {
    let defaultResolver = field.resolve;

    field.resolve = async (parent, args, context, info) => {
      let entityId;

      if (defaultResolver) {
        entityId = await defaultResolver(parent, args, context, info);
      } else if (args.courseId) {
        entityId = args.courseId;
      } else {
        entityId = parent[field.name];
      }

      if (!entityId) return null;

      console.log('hi2', entityId);
      if (typeof entityId === 'string')
        return this.loadEntry(entityId, context.callZome);
      else return entityId.map(id => this.loadEntry(id, context.callZome));
    };
  }

  async loadEntry(entityId, callZome) {
    console.log('hi3', entityId);

    const entryResult = await callZome(
      INSTANCE_NAME,
      ZOME_NAME,
      'get_entry'
    )({
      address: entityId
    });
    console.log('hi4', entryResult);

    const entry = parseEntry(entryResult);
    console.log('hi5', entry);

    return { id: entityId, ...entry };
  }
}
