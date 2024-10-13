// ESM
import { faker } from '@faker-js/faker';


//   {
//     "uid": "0",
//     "first": "Ethan",
//     "last": "Armstrong",
//     "contact": [
//       "9494244530",
//       "warmst@uw.edu"
//     ],
//     "skills": [
//       "programming"
//     ],
//     "wants": [
//       "ui"
//     ]
//   },


export function createRandomUser() {
  return {
    userId: faker.string.uuid(),
    first: faker.person.firstName(),
    last: faker.person.lastName(),
    contact: [
        faker.phone.number(),
        faker.internet.email()
    ],
    skills: [
        faker.word.noun()
    ],
    wants: [
        faker.word.noun()
    ],
    image: faker.image.avatar()
  };
}

export const fakeUsers = faker.helpers.multiple(createRandomUser, {
  count: 10,
});