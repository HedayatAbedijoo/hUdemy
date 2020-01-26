import { connect } from '@holochain/hc-web-client';
import { HOST_URL } from './config';

let connection = undefined;

export async function getConnection() {
  if (connection) return connection;

  const { callZome } = await connect({ url: HOST_URL });
  connection = (instance, zome, fnName) => async params => {
    console.log(
      `Calling zome function: ${instance}/${zome}/${fnName} with params`,
      params
    );

    const result = await callZome(instance, zome, fnName)(params);

    console.log(
      `Zome function ${instance}/${zome}/${fnName} with params returned`,
      result
    );

    return result;
  };

  return connection;
}
