import { makeExecutableSchema } from "graphql-tools";
import { ApolloClient, InMemoryCache } from "apollo-boost";

import { resolvers } from "./resolvers";
import { typeDefs } from "./schema";
import { LoadEntityDirective } from "./directive";
import { getConnection } from "../connection";

let client = undefined;

export async function getClient() {
  if (client) return client;

  const connection = await getConnection();

  const schema = makeExecutableSchema({
    typeDefs,
    resolvers,
    schemaDirectives: {
      loadEntry: LoadEntityDirective
    }
  });

  const link = new SchemaLink({ schema, context: { callZome: connection } });

  client = new ApolloClient({
    cache: new InMemoryCache(),
    connectToDevTools: true,
    link
  });
}
