import { create } from 'zustand'
import axios from 'axios'

export interface Todo {
    id: string
    title: string
    completed: boolean
}

interface JobStatus {
    id: string
    status: 'pending' | 'completed' | 'failed'
}

interface TodoState {
    todos: Todo[]
    loadingDoneIds: string[]
    loadingDeleteIds: string[]
    fetchTodos: () => Promise<void>
    createTodo: (title: string) => Promise<void>
    deleteTodo: (id: string) => Promise<void>
    toggleTodo: (id: string) => Promise<void>
    remainingTodos: () => number
}

const API = axios.create({ baseURL: import.meta.env.VITE_API_URL || '' })

async function pollJob(jobId: string) {
    let status: JobStatus['status']
    do {
        const res = await API.get<JobStatus>(`/jobs/${jobId}`)
        status = res.data.status
        if (status === 'completed') return
        if (status === 'failed') throw new Error(`Job ${jobId} failed`)
        await new Promise(r => setTimeout(r, 500))
    } while (true)
}

export const useTodos = create<TodoState>((set, get) => ({
    todos: [],
    loadingDoneIds: [],
    loadingDeleteIds: [],

    fetchTodos: async () => {
        const res = await API.get<Todo[]>('/todos?page=1&limit=100')
        set({ todos: res.data })
    },

    createTodo: async (title) => {
        const { data: job } = await API.post<{ id: string; status: string }>('/todos', { title })
        await pollJob(job.id)
        await get().fetchTodos()
    },

    deleteTodo: async (id) => {
        set(state => ({ loadingDeleteIds: [...state.loadingDeleteIds, id] }));
        try {
            const { data: job } = await API.delete<{ id: string; status: string }>(`/todos/${id}`)
            await pollJob(job.id)
            set(state => ({ todos: state.todos.filter(t => t.id !== id) }))
        } finally {
            set(state => ({ loadingDeleteIds: state.loadingDeleteIds.filter(x => x !== id) }));
        }
    },

    toggleTodo: async (id) => {
        set(state => ({ loadingDoneIds: [...state.loadingDoneIds, id] }));
        try {
            const { data: job } = await API.post<{ id: string; status: string }>(`/todos/${id}/toggle`)
            await pollJob(job.id)
            set(state => ({
                todos: state.todos.map(t =>
                    t.id === id ? { ...t, completed: !t.completed } : t
                )
            }))
        } finally {
            set(state => ({ loadingDoneIds: state.loadingDoneIds.filter(x => x !== id) }));
        }
    },

    remainingTodos: () => get().todos.filter(todo => !todo.completed).length
}))