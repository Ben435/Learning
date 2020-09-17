import { addTodo, getTodos } from './_todo-service';

export async function post(req, res, _next) {
    const { content, title } = req.body || {};

    await addTodo({ title, content });

    res.statusCode = 204;
    return res.end();
}

export async function get(_req, res, _next) {
    const todos = await getTodos();

    res.setHeader('Content-Type', 'application/json');
    return res.end(JSON.stringify(todos));
}
