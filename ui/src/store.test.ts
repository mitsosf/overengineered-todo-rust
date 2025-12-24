import { create } from 'zustand';
import axios from 'axios';
import { useTodos, Todo, TodoState } from './store';

jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('useTodos', () => {
    let store: TodoState;

    beforeEach(() => {
        store = useTodos.getState();
    });

    afterEach(() => {
        jest.clearAllMocks();
    });

    it('fetches todos', async () => {
        const todos: Todo[] = [
            { id: '1', title: 'Test todo 1', completed: false },
            { id: '2', title: 'Test todo 2', completed: true },
        ];
        mockedAxios.get.mockResolvedValueOnce({ data: todos });

        await store.fetchTodos();

        expect(mockedAxios.get).toHaveBeenCalledWith('/todos?page=1&limit=100');
        expect(store.todos).toEqual(todos);
    });

    it('creates a todo', async () => {
        const title = 'New todo';
        mockedAxios.post.mockResolvedValueOnce({ data: { id: '1', status: 'completed' } });
        mockedAxios.get.mockResolvedValueOnce({ data: [] });

        await store.createTodo(title);

        expect(mockedAxios.post).toHaveBeenCalledWith('/todos', { title });
        expect(store.fetchTodos).toHaveBeenCalled();
    });

    it('deletes a todo', async () => {
        const id = '1';
        mockedAxios.delete.mockResolvedValueOnce({ data: { id, status: 'completed' } });

        await store.deleteTodo(id);

        expect(mockedAxios.delete).toHaveBeenCalledWith(`/todos/${id}`);
        expect(store.todos).not.toContainEqual(expect.objectContaining({ id }));
    });

    it('toggles a todo', async () => {
        const id = '1';
        const todo: Todo = { id, title: 'Test todo', completed: false };
        store.todos.push(todo);
        mockedAxios.post.mockResolvedValueOnce({ data: { id, status: 'completed' } });

        await store.toggleTodo(id);

        expect(mockedAxios.post).toHaveBeenCalledWith(`/todos/${id}/toggle`);
        expect(store.todos).toContainEqual(expect.objectContaining({ id, completed: true }));
    });

    it('counts remaining todos', () => {
        store.todos.push({ id: '1', title: 'Test todo 1', completed: false });
        store.todos.push({ id: '2', title: 'Test todo 2', completed: true });

        const remaining = store.remainingTodos();

        expect(remaining).toBe(1);
    });
});