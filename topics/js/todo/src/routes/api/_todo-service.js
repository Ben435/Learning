
const todos = [];

const dummyItems = [
    { title: 'test', content: 'hello world!' },
    { content: 'bye bye!' },
];

todos.push(...dummyItems);


export const addTodo = async(todo) => {
    todos.push(todo);
};

export const getTodos = async() => todos;
