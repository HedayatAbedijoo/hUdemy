export function parseResponse(response) {
  return response.hasOwnProperty("Ok") ? response.Ok : response;
}

export function parseEntry(entry) {
  return JSON.parse(this.parseResponse(entry).App[1]);
}
