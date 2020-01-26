import { connect } from "@holochain/hc-web-client";
import { HOST_URL } from "./config";

let connection = undefined;

export async function getConnection() {
  if (connection) return connection;

  const { callZome } = await connect({ url: HOST_URL });
  connection = callZome;

  return connection;
}
