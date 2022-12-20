import React, { useState } from "react";
import classnames from "classnames";
import { Habit, HabitUpdate } from "../habit";

export function HabitItem({
  habit,
}: {
  habit: Habit;

}) {
  const { description } = habit;
  const [editing, setEditing] = useState(false);

  const handleDoubleClick = () => {
    setEditing(true);
  };

  return (
    <li
      className={classnames({
        approved: habit.approved,
        editing,
      })}
    >
    </li>
  );
}
