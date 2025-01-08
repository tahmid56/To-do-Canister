import React, { useEffect, useState } from 'react';
import { to_do_backend } from '../../declarations/to-do-backend';


function App() {
  const [todos, setTodos] = useState([]);
  const [description, setDescription] = useState('');
  const [isUpdating, setIsUpdating] = useState(false);
  const [currentId, setCurrentId] = useState(null);

  useEffect(() => {
    fetchTodos();
  }, []);

  const fetchTodos = async () => {
    const todoList = await to_do_backend.get_all_todo_list();
    setTodos(todoList);
  };

  const addTodo = async () => {
    if (description) {
      await to_do_backend.add_todo(description);
      setDescription('');
      fetchTodos();
    }
  };

  const updateTodo = async (id, newDescription, completed) => {
    await to_do_backend.update_todo(id, newDescription, completed);
    fetchTodos();
  };

  const deleteTodo = async (id) => {
    await to_do_backend.delete_todo(id);
    fetchTodos();
  };

  const handleEdit = (todo) => {
    setDescription(todo.description);
    setCurrentId(todo.id);
    setIsUpdating(true);
  };

  const handleSubmit = () => {
    if (isUpdating) {
      updateTodo(currentId, description, false);
      setIsUpdating(false);
      setCurrentId(null);
    } else {
      addTodo();
    }
  };

  return (
    <div className="App">
      <h1>To-Do List</h1>
      <input
        type="text"
        value={description}
        onChange={(e) => setDescription(e.target.value)}
        placeholder="Add a new task"
      />
      <button onClick={handleSubmit}>
        {isUpdating ? 'Update' : 'Add'}
      </button>
      <ul>
        {todos.map((todo) => (
          <li key={todo.id}>
            <button onClick={() => updateTodo(todo.id, todo.description, !todo.completed)}>
              {todo.completed ? 'Mark as Incomplete' : 'Mark as Complete'}
            </button>
            <p style={{"width": "100%"}}>{todo.description}</p>
            
            <button onClick={() => handleEdit(todo)}>Edit</button>
            <button onClick={() => deleteTodo(todo.id)}>Delete</button>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default App;
