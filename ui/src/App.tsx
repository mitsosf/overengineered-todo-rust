import { useEffect, useState, type FormEvent } from 'react'
import { useTodos } from './store'
import {PlusCircle, Trash2, CheckCircle} from "lucide-react"

export function App() {
    const { todos, loadingDoneIds, loadingDeleteIds, fetchTodos, createTodo, deleteTodo, toggleTodo, remainingTodos } = useTodos()
    const [title, setTitle] = useState('')
    const [loading, setLoading] = useState(false)

    useEffect(() => { fetchTodos() }, [fetchTodos])

    const handleTodo = async (e: FormEvent<HTMLFormElement>) => {
        setLoading(prev => !prev)
        e.preventDefault()
        if (!title.trim()) {
            console.warn('Title cannot be empty')
            return
        }
        await createTodo(title.trim())
        setTitle('')
        setLoading(prev => !prev)
    }

    return (

        <div className="max-w-md mx-auto bg-white rounded-xl shadow-md overflow-hidden p-6">
            <h1 className="text-2xl font-bold text-center text-gray-800 mb-6">Overengineered 🦀 Todos</h1>
            <div className="text-center text-blue-500 mb-4">
                <a target='_blank' href='https://github.com/mitsosf/overengineered-todo-rust'>Github: mitsosf/overengineered-todo-rust</a>
            </div>
            <form onSubmit={e => handleTodo(e)}>
                <div className="flex items-center mb-6">
                    <input
                        type="text"
                        onChange={(e) => setTitle(e.target.value)}
                        value={title}
                        placeholder="Add a new todo..."
                        className="flex-grow px-4 py-2 border rounded-l-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                    />
                    <button
                        type="submit"
                        className="bg-blue-500 hover:bg-blue-600 text-white px-4 py-2 rounded-r-lg flex items-center transition-colors"
                        disabled={loading}
                    >
                        <PlusCircle className="h-5 w-5 mr-1"/>
                        Add
                    </button>
                </div>
            </form>

            {/* Todo List */}
            <div className="space-y-3">
                {todos.map((todo) => {
                    const isLoadingDone = loadingDoneIds.includes(todo.id);
                    const isLoadingDelete = loadingDeleteIds.includes(todo.id);
                    return (
                    <div key={todo.id} className="bg-gray-50 rounded-lg p-4 shadow-sm border border-gray-100">
                        <div className="flex items-center justify-between">
                            <p className={`${todo.completed ? "line-through text-gray-500" : "text-gray-800"}`}>{todo.title}</p>
                            <div className="flex space-x-2">
                                <button
                                    className="text-green-600 hover:text-green-800 p-1 rounded transition-colors"
                                    aria-label="Mark as done"
                                    disabled={isLoadingDone}
                                    onClick={() =>toggleTodo(todo.id)}
                                >
                                    <CheckCircle className="h-5 w-5" />
                                </button>
                                <button
                                    className="text-red-600 hover:text-red-800 p-1 rounded transition-colors"
                                    aria-label="Delete todo"
                                    disabled={isLoadingDelete}
                                    onClick={() => deleteTodo(todo.id)}
                                >
                                    <Trash2 className="h-5 w-5" />
                                </button>
                            </div>
                        </div>
                    </div>
                )})}
            </div>

            {/* Todo Stats */}
            <div className="mt-6 text-sm text-gray-500 text-center">{remainingTodos()} tasks left to complete</div>
        </div>
    )
}

export default App;