import axios from 'axios';

export const createNewTodo = async(content) => axios.post('api/todo', { content });

export const editTodo = async(id, newTodo) => axios.post(`api/todo/${id}`, newTodo);

export const fetchTodos = async() => axios.get('api/todo');

export const fetchTodo = async(id) => axios.get(`api/todo/${id}`);

export const deleteTodo = async(id) => axios.delete(`api/todo/${id}`);
