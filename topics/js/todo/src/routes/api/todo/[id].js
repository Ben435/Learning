import { getTodo, editTodo, deleteTodo } from "../_todo-service";

export async function get(req, res, next) {
    const { id } = req.params;

    const todo = await getTodo(id);

    if (todo) {
        return res.end(JSON.stringify(todo));
    }

    next();
}

export async function post(req, res, _next) {
    const { id } = req.params;
    const { content, title } = req.body || {};

    const newTodo = await editTodo(id, { content, title });

    return res.end(JSON.stringify(newTodo));
}

export async function del(req, res, _next) {
    const { id } = req.params;

    await deleteTodo(id);

    res.statusCode = 204;
    return res.end();
}
