import axios from 'axios';

export const createNewTodo = async(content) => axios.post('api/todo', { content });

export const fetchTodos = async() => axios.get('api/todo');
