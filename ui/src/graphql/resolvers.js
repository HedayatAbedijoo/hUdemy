import { INSTANCE_NAME, ZOME_NAME } from '../config';
import { parseResponse } from '../utils';

export const resolvers = {
  Query: {
    async allCourses(_, __, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'get_courses'
      )({});

      console.log(result);
      return parseResponse(result);
    }
  },
  Course: {
    async students(parent) {
      return [];
    }
  },
  Module: {
    async contents() {
      return [];
    }
  },
  Mutation: {
    async createCourse(_, { title }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'create_course'
      )({
        title
      });

      return parseResponse(result);
    },
    async updateCourse(_, { title, courseId, modulesAddresses }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'update_course'
      )({
        title,
        course_address: courseId,
        modules_addresses: modulesAddresses
      });

      return parseResponse(result);
    },
    async deleteCourse(_, { courseId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'delete_course'
      )({
        course_address: courseId
      });

      return parseResponse(result);
    },
    async createModule(_, { courseId, title }, { callZome }) {
      console.log('result32');
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'create_module'
      )({
        course_address: courseId,
        title
      });
      console.log('result', result);
      return parseResponse(result);
    },
    async updateModule(_, { moduleId, title }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'update_module'
      )({
        module_address: moduleId,
        title
      });

      return parseResponse(result);
    },
    async deleteModule(_, { moduleId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'delete_module'
      )({
        module_address: moduleId
      });

      return parseResponse(result);
    },
    async createContent(_, { content }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'create_content'
      )({});

      return parseResponse(result);
    }
  }
};
