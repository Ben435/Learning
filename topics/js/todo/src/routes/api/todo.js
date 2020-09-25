import { addTodo, getTodos } from './_todo-service';

export async function post(req, res, _next) {
    const { content, title } = req.body || {};

    const newTodo = await addTodo({ title, content });

    res.statusCode = 201;
    res.setHeader('Content-Type', 'application/json');
    return res.end(JSON.stringify(newTodo));
}

export async function get(_req, res, _next) {
    const todos = await getTodos();

    res.setHeader('Content-Type', 'application/json');
    return res.end(JSON.stringify(todos));
}
