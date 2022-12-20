import React, { useState } from "react";
import { Todo, TodoUpdate } from "../todo";
import { Habit, HabitUpdate } from "../habit";
import Footer from "./footer";
import TodoList from "./todo-list";
import HabitList from "./habit-list";

const MainSection = ({
  todos,
  habits,
  onUpdateTodo,
  onDeleteTodos,
  onCompleteTodos,
}: {
  todos: Todo[];
  habits: Habit[];
  onUpdateTodo: (update: TodoUpdate) => void;
  onDeleteTodos: (ids: string[]) => void;
  onCompleteTodos: (completed: boolean, ids: string[]) => void;
}) => {
  const todosCount = todos.length;
  const completed = todos.filter((todo) => todo.completed);
  const completedCount = completed.length;
  const toggleAllValue = completedCount === todosCount;

  const [filter, setFilter] = useState("All");

  const filteredTodos = todos.filter((todo) => {
    if (filter === "All") {
      return true;
    }
    if (filter === "Active") {
      return !todo.completed;
    }
    if (filter === "Completed") {
      return todo.completed;
    }
    throw new Error("Unknown filter: " + filter);
  });

  const handleCompleteAll = () => {
    const completed = !toggleAllValue;
    onCompleteTodos(
      completed,
      todos.map((todo) => todo.id)
    );
  };

  return (
    <section className="main">
      {todosCount > 0 && (
        <span>
          <input
            className="toggle-all"
            type="checkbox"
            defaultChecked={toggleAllValue}
          />
          <label onClick={handleCompleteAll} />
        </span>
      )}z
      <TodoList
        todos={filteredTodos}
        onUpdateTodo={onUpdateTodo}
        onDeleteTodo={(id) => onDeleteTodos([id])}
      />
      <HabitList
        habits={habits}
      />
      {todos.length > 0 && (
        <Footer
          completed={completedCount}
          active={todosCount - completedCount}
          onDeleteCompleted={() =>
            onDeleteTodos(completed.map((todo) => todo.id))
          }
          currentFilter={filter}
          onFilter={setFilter}
        />
      )}
    </section>
  );
};

export default MainSection;
