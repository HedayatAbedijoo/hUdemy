import gql from 'graphql-tag';

export const GET_ALL_COURSES = gql`
  query GetAllCourses {
    allCourses {
      id
      title
      teacher_address
      students
    }
  }
`;

export const GET_COURSE_INFO = gql`
  query GetCourseInfo($courseId: String) {
    course(courseId: $courseId) {
      id
      title
      students
      teacher_address
      modules {
        id
        title
        contents {
          id
          title
          description
          url
        }
      }
    }
  }
`;

export const CREATE_COURSE = gql`
  mutation CreateCourse($title: String) {
    createCourse(title: $title) {
      id
      title
      teacher_address
    }
  }
`;

export const CREATE_MODULE = gql`
  mutation CreateModule($courseId: ID!, $title: String!) {
    createModule(courseId: $courseId, title: $title) {
      id
      title
    }
  }
`;
