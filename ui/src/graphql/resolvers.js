export const resolvers = {
  Query: {
    async allCourses(_, __, { callZome }) {
      return callZome(INSTANCE_NAME, ZOME_NAME, "get_courses", {});
    }
  },
  Course: {
    async students(parent) {
      return [];
    }
  }
};
