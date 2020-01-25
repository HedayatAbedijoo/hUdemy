import gql from "graphql-tag";

export const typeDefs = gql`
  type Course {
    id: ID!
    title: String!
    modules: [Module!]!
    teacher_address: ID!
  }

  type Module {
    id: ID!
    title: String!
    contents: [Content!]!
  }

  type Content {
    id: ID!
    title: String!
    description: String!
    url: String!
  }

  type Query {
    allCourses: [Course!]!
  }
`;

/*     myCourses: [Course!]!
    myAddress: ID!
 */
