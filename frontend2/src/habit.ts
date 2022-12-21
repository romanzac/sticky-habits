// This file defines Habit domain type in TypeScript, and a related helper
// function to get all Habits.

import { StickyHabits } from "./near-interface";
import HabitList from "./components/habit-list";

export type Habit = {
  description: string,
  deadline: string,
  deposit: number,
  beneficiary: string,
  evidence: string,
  approved: boolean,

};

export type HabitUpdate = Partial<Habit> & Pick<Habit, "description">;

export function listHabits(contract: StickyHabits) {
  let result: Habit[] = [];
  contract.getUserHabits().then(res => {
    result = res;
  });
  return result
}
