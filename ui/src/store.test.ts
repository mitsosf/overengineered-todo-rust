import { create } from 'zustand'
import axios from 'axios'
import { useTodos, Todo, TodoState } from './store'

jest.mock('axios')
const mockedAxios = axios as jest.Mocked<typeof axios>

describe('useTodos', () => {
    let store: TodoState

    beforeEach(() => {
        store = useTodos()
    })

    afterEach(() => {
        jest.clearAllMocks()
    })

    it('fetches todos', async () => {
        const todos: Todo[] = [
            { id: '1', title: 'Todo 1', completed: false },
            { id: '2', title: 'Todo 2', completed: true },
        ]
        mockedAxios.get.mockResolvedValueOnce({ data: todos })

        await store.fetchTodos()

        expect(mockedAxios.get).toHaveBeenCalledWith('/todos?page=1&limit=100')
        expect(store.todos).toEqual(todos)
    })

    it('creates a todo', async () => {
        const title = 'New Todo'
        mockedAxios.post.mockResolvedValueOnce({ data: { id: '3', status: 'completed' } })
        mockedAxios.get.mockResolvedValueOnce({ data: { status: 'completed' } })

        await store.createTodo(title)

        expect(mockedAxios.post).toHaveBeenCalledWith('/todos', { title })
        expect(mockedAxios.get).toHaveBeenCalledWith('/jobs/3')
    })

    it('deletes a todo', async () => {
        const id = '1'
        mockedAxios.delete.mockResolvedValueOnce({ data: { id: '4', status: 'completed' } })
        mockedAxios.get.mockResolvedValueOnce({ data: { status: 'completed' } })

        await store.deleteTodo(id)

        expect(mockedAxios.delete).toHaveBeenCalledWith(`/todos/${id}`)
        expect(mockedAxios.get).toHaveBeenCalledWith('/jobs/4')
    })

    it('toggles a todo', async () => {
        const id = '1'
        mockedAxios.post.mockResolvedValueOnce({ data: { id: '5', status: 'completed' } })
        mockedAxios.get.mockResolvedValueOnce({ data: { status: 'completed' } })

        await store.toggleTodo(id)

        expect(mockedAxios.post).toHaveBeenCalledWith(`/todos/${id}/toggle`)
        expect(mockedAxios.get).toHaveBeenCalledWith('/jobs/5')
    })

    it('counts remaining todos', () => {
        store.todos = [
            { id: '1', title: 'Todo 1', completed: false },
            { id: '2', title: 'Todo 2', completed: true },
        ]

        const remaining = store.remainingTodos()

        expect(remaining).toBe(1)
    })
})