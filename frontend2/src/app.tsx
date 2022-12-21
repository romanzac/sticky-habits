import { nanoid } from "nanoid";
import React from "react";
import { Replicache } from "replicache";
import { useSubscribe } from "replicache-react";

import { Wallet } from './near-wallet';
import { StickyHabits } from './near-interface';

import { M } from "./mutators";
import { listTodos, TodoUpdate } from "./todo";

import { listHabits, HabitUpdate } from "./habit";

import Header from "./components/header";
import MainSection from "./components/main-section";
import "todomvc-app-css/index.css";



// This is the top-level component for our app.
const App = ({ rep }: { rep: Replicache<M> }) => {
  console.log("Test test testu")

  // Subscribe to all todos and sort them.
  const todos = useSubscribe(rep, listTodos, [], [rep]);
  todos.sort((a, b) => a.sort - b.sort);

  const CONTRACT_ADDRESS: string = process.env.CONTRACT_NAME!;

  // Wallet instance
  // @ts-ignore
  const wallet = new Wallet({ createAccessKeyFor: CONTRACT_ADDRESS })

  // Logic for interacting with the contract
  const stickyHabits = new StickyHabits({ contractId: CONTRACT_ADDRESS, walletToUse: wallet });

  // Get all habits
  const habits = listHabits(stickyHabits);

  // Define event handlers and connect them to Replicache mutators. Each
  // of these mutators runs immediately (optimistically) locally, then runs
  // again on the server-side automatically.
  const handleNewItem = (text: string) =>
    rep.mutate.createTodo({
      id: nanoid(),
      text,
      completed: false,
    });

  const handleUpdateTodo = (update: TodoUpdate) =>
    rep.mutate.updateTodo(update);

  const handleDeleteTodos = (ids: string[]) => {
    for (const id of ids) {
      rep.mutate.deleteTodo(id);
    }
  };

  const handleCompleteTodos = (completed: boolean, ids: string[]) => {
    for (const id of ids) {
      rep.mutate.updateTodo({
        id,
        completed,
      });
    }
  };

  // Render app.

  return (
    <div>
      <Header onNewItem={handleNewItem} />
      <MainSection
        todos={todos}
        habits={habits}
        onUpdateTodo={handleUpdateTodo}
        onDeleteTodos={handleDeleteTodos}
        onCompleteTodos={handleCompleteTodos}
      />
    </div>
  );
};

export default App;
