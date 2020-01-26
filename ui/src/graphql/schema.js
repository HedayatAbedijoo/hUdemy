import gql from 'graphql-tag';

export const typeDefs = gql`
  directive @loadEntry on FIELD_DEFINITION

  type Course {
    id: ID!
    title: String!
    modules: [Module!]! @loadEntry
    teacher_address: ID!
    students: [ID!]!
  }

  type Module {
    id: ID!
    title: String!
    contents: [Content!]! @loadEntry
  }

  type Content {
    id: ID!
    title: String!
    description: String!
    url: String!
  }

  type Query {
    allCourses: [Course!]! @loadEntry
    course(courseId: ID!): Course! @loadEntry
  }

  type Mutation {
    createCourse(title: String!): Course! @loadEntry
    updateCourse(
      courseId: ID!
      title: String!
      modulesIds: [ID!]!
    ): Course! @loadEntry
    deleteCourse(courseId: ID!): ID!
    createModule(courseId: ID!, title: String!): Module! @loadEntry
    updateModule(moduleId: ID!, title: String!): Module! @loadEntry
    deleteModule(moduleId: ID!): Module! @loadEntry
  }
`;

/*     myCourses: [Course!]!
    myAddress: ID!
 */
