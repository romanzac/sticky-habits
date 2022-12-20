// This file defines Habit domain type in TypeScript, and a related helper
// function to get all Habits.

import { StickyHabits } from "./near-interface";

export type Habit = {
  description: string,
  deadline: string,
  deposit: number,
  beneficiary: string,
  evidence: string,
  approved: boolean,

};

export type HabitUpdate = Partial<Habit> & Pick<Habit, "description">;

export async function listHabits(contract: StickyHabits) {
  return await contract.getUserHabits();
}
