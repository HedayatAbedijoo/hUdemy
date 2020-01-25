import { ApolloClient } from "apollo-boost";
import { resolvers } from "./resolvers";
import { typeDefs } from "./schema";
import { makeExecutableSchema } from "graphql-tools";
import { LoadEntityDirective } from "./directive";

export const schema = makeExecutableSchema({
  typeDefs,
  resolvers,
  schemaDirectives: {
    loadEntry: LoadEntityDirective
  }
});

const link = new SchemaLink({schema, context: {}})

export const client = new ApolloClient({

});
