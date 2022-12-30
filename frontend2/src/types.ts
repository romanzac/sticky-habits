// You can include shared src/types in a separate file
// and then use them in any component by importing them. For
// example, to import the interface below do:
//
// import { User } from 'path/to/src';

export type User = {
  id: number
  name: string
}

export type Habit = {
  description: string,
  deadline: string,
  deposit: number,
  beneficiary: string,
  evidence: string,
  approved: boolean,

};