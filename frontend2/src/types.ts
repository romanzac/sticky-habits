// You can include shared src/types in a separate file
// and then use them in any component by importing them. For
// example, to import the interface below do:
//
// import { User } from 'path/to/src';

export type User = {
  id: number
  name: string
}
// TODO: extend Habit datatype with an ID for each habit
export type Habit = {
  id: string,
  description: string,
  deadline: string,
  deposit: number,
  beneficiary: string,
  evidence: string,
  approved: boolean,

};