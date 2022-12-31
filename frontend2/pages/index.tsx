// import Link from 'next/link'
// import Layout from '../components/Layout'

// const IndexPage = () => (
//   <Layout title="Home | Next.js + TypeScript Example">
//     <h1>Hello Next.js 👋</h1>
//     <p>
//       <Link href="/about">About</Link>
//     </p>
//   </Layout>
// )
//
// export default IndexPage

import React, { useState } from 'react';
import { NextPage } from 'next';
import { Habit } from '../src/types';


// // Todo interface representing a todo item
// interface Todo {
//   id: number;
//   text: string;
//   completed: boolean;
// }

const Home: NextPage = () => {
  // Initialize the list of todos in state
  const [habits, setHabits] = useState<Habit[]>([
    { description: 'Go to gym everyday', deadline: '1', deposit: 1, beneficiary: 'pepa.testnet',
    evidence: '', approved: false},
    { description: 'Cook for grandmother once a week', deadline: '1', deposit: 1, beneficiary: 'pepa.testnet',
      evidence: '', approved: false}
  ]);

  // Function to add a new todo
  const handleAddHabit = (description: string, deadline: string, deposit: number, beneficiary: string) => {
    const newHabit: Habit = {
      description,
      deadline,
      deposit,
      beneficiary,
      evidence: '',
      approved: false
    };
    setHabits([...habits, newHabit]);
  };

  // Function to mark a todo as completed
  const handleToggleCompleted = (description: string) => {
    const updatedHabits = habits.map((habit) => {
      if (habit.description === description) {
        return { ...habit, approved: !habit.approved };
      }
      return habit;
    });
    setHabits(updatedHabits);
  };

  // Function to delete a todo
  const handleDeleteTodo = (description: string) => {
    const updatedHabits = habits.filter((habit) => habit.description !== description);
    setHabits(updatedHabits);
  };

  // Function to clear completed todos
  const handleClearCompleted = () => {
    const updatedHabits = habits.filter((habit) => !habit.approved);
    setHabits(updatedHabits);
  };

  return (
    <div>
      <h1>My Habits</h1>
      <form
        onSubmit={(event) => {
          event.preventDefault();
          const description = event.currentTarget.elements[0] as HTMLInputElement;
          const deadline = event.currentTarget.elements[1] as HTMLInputElement;
          const deposit = event.currentTarget.elements[2] as HTMLInputElement;
          const beneficiary = event.currentTarget.elements[3] as HTMLInputElement;
          handleAddHabit(description.value, deadline.value, parseInt(deposit.value), beneficiary.value);
          description.value = '';
          deadline.value = '';
          deposit.value = '';
          beneficiary.value = '';
        }}
      >
        <input type="text" name="description" />
        <input type="text" name="deadline" />
        <input type="number" name="deposit" />
        <input type="text" name="beneficiary" />
        <button type="submit">Add Habit</button>
      </form>
      <table className='table table-striped'>
        <thead>
        <tr>
          <th scope="col">Description</th>
          <th scope="col">Deadline</th>
          <th scope="col">Deposit</th>
          <th scope="col">Beneficiary</th>
          <th scope="col">Evidence</th>
          <th scope="col">Approved</th>
        </tr>
        </thead>
        {habits.map((habit) => (
          <tr key={habit.description}>
            {habit.description}
            {habit.deadline}
            {habit.deposit}
            {habit.beneficiary}
            {habit.evidence}
            {habit.approved}
            <input
              type="checkbox"
              checked={habit.approved}
              onChange={() => handleToggleCompleted(habit.description)}
            />
          </tr>
        ))}
      </table>
    </div>
  );
};

export default Home
