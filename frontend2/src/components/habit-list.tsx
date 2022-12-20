import React from "react";
import { Habit, HabitUpdate } from "../habit";
import { HabitItem } from "./habit-item";

const HabitList = ({
  habits,
}: {
  habits: Habit[];
}) => {
  return (
    <ul className="todo-list">
      {habits.map((habit) => (
        <HabitItem
          habit={habit}
          key={habit.description}
        />
      ))}
    </ul>
  );
};

export default HabitList;
