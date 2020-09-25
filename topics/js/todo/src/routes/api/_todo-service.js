
let todos = [];

const dummyItems = [
    { id: '1', title: 'test', content: 'hello world!' },
    { id: '2', content: 'bye bye!' },
];

let i = 3;

todos.push(...dummyItems);

export const addTodo = async(todo) => {
    const { title, content } = todo;
    const newTodo = { 
        id: '' + i++,
        title,
        content,
    };

    todos = [...todos, newTodo];

    return newTodo;
};

export const editTodo = async(id, newTodo) => {
    const todo = todos.find(todo => todo.id === id);

    if (todo) {
        const updatedTodo = {
            ...todo,
            title: newTodo.title,
            content: newTodo.content,
        }
        todos = todos.map((cur) => cur.id === id ? updatedTodo : cur);

        return updatedTodo;
    }

    return false;
}

export const getTodos = async() => todos;

export const getTodo = async(id) => {
    const todo = todos.find(todo => todo.id === id);

    if (todo) {
        return todo;
    }
    return null;
}

export const deleteTodo = async(id) => {
    todos = todos.filter(todo => todo.id === id);
}
