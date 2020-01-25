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
`