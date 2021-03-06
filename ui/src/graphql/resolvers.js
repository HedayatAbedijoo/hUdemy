import { INSTANCE_NAME, ZOME_NAME } from '../config';
import { parseResponse } from '../utils';

export const resolvers = {
  Query: {
    async courses(_, { filter }, { callZome }) {
      const fnName =
        filter === 'enrolled-courses'
          ? 'get_my_enrolled_courses'
          : filter === 'my-courses'
          ? 'get_my_courses'
          : 'get_all_courses';

      const result = await callZome(INSTANCE_NAME, ZOME_NAME, fnName)({});

      return parseResponse(result);
    },
    async myAddress(_, __, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'get_my_address'
      )({});

      return parseResponse(result);
    }
  },
  Course: {
    async students(parent, _, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'get_all_students'
      )({
        course_address: parent.id
      });

      return parseResponse(result);
    }
  },
  Module: {
    async contents(parent, _, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'get_contents'
      )({
        module_address: parent.id
      });

      return parseResponse(result);
    }
  },
  Mutation: {
    async createCourse(_, { title }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'create_course'
      )({
        timestamp: getTimestamp(),
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
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'create_module'
      )({
        timestamp: getTimestamp(),
        course_address: courseId,
        title
      });

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
    async createContent(_, { content, moduleId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'create_content'
      )({
        timestamp: getTimestamp(),
        name: content.name,
        module_address: moduleId,
        url: content.url,
        description: content.description
      });

      return parseResponse(result);
    },
    async updateContent(_, { content, contentId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'update_content'
      )({
        name: content.name,
        content_address: contentId,
        url: content.url,
        description: content.description
      });

      return parseResponse(result);
    },
    async deleteContent(_, { contentId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'delete_content'
      )({
        content_address: contentId
      });

      return parseResponse(result);
    },
    async enrolInCourse(_, { courseId }, { callZome }) {
      const result = await callZome(
        INSTANCE_NAME,
        ZOME_NAME,
        'enrol_in_course'
      )({
        course_address: courseId
      });

      parseResponse(result);
      return courseId;
    }
  }
};

function getTimestamp() {
  return Math.floor(Date.now() / 1000);
}
