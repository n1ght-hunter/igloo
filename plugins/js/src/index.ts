import {
    createApp,
    Text,
    Column,
    Button,
    Row,
    Container,
    Length,
    TextInput,
    Checkbox,
    ProgressBar,
    Rule,
    Space,
    Padding,
    Scrollable,
} from "igloo-ts"

// Task type
interface Task {
    id: number
    text: string
    completed: boolean
}

// App state
interface State {
    tasks: Task[]
    inputText: string
    nextId: number
    filter: 'all' | 'active' | 'completed'
}

// Messages
type Msg =
    | { type: 'inputChanged', value: string }
    | { type: 'addTask' }
    | { type: 'toggleTask', id: number }
    | { type: 'deleteTask', id: number }
    | { type: 'setFilter', filter: 'all' | 'active' | 'completed' }
    | { type: 'clearCompleted' }

const Application = createApp<State, Msg>({
    init: () => ({
        tasks: [
            { id: 1, text: 'Learn igloo-ts', completed: true },
            { id: 2, text: 'Build a cool app', completed: false },
            { id: 3, text: 'Share with others', completed: false },
        ],
        inputText: '',
        nextId: 4,
        filter: 'all',
    }),

    update: (state, msg) => {
        switch (msg.type) {
            case 'inputChanged':
                return { ...state, inputText: msg.value }

            case 'addTask':
                if (state.inputText.trim() === '') return state
                return {
                    ...state,
                    tasks: [...state.tasks, {
                        id: state.nextId,
                        text: state.inputText.trim(),
                        completed: false,
                    }],
                    inputText: '',
                    nextId: state.nextId + 1,
                }

            case 'toggleTask':
                return {
                    ...state,
                    tasks: state.tasks.map(t =>
                        t.id === msg.id ? { ...t, completed: !t.completed } : t
                    ),
                }

            case 'deleteTask':
                return {
                    ...state,
                    tasks: state.tasks.filter(t => t.id !== msg.id),
                }

            case 'setFilter':
                return { ...state, filter: msg.filter }

            case 'clearCompleted':
                return {
                    ...state,
                    tasks: state.tasks.filter(t => !t.completed),
                }
        }
    },

    view: (state) => {
        const completedCount = state.tasks.filter(t => t.completed).length
        const totalCount = state.tasks.length
        const progress = totalCount > 0 ? completedCount / totalCount : 0

        // Filter tasks based on current filter
        const filteredTasks = state.tasks.filter(t => {
            if (state.filter === 'active') return !t.completed
            if (state.filter === 'completed') return t.completed
            return true
        })

        // Build task list
        const taskList = Column.new().spacing(8)
        for (const task of filteredTasks) {
            taskList.push(
                Row.new()
                    .spacing(10)
                    .push(
                        Checkbox.new(task.completed)
                            .label(task.text)
                            .onToggle(() => ({ type: 'toggleTask' as const, id: task.id }))
                    )
                    .push(Space.new().width(Length.fill()))
                    .push(
                        Button.new(Text.new('×').size(16))
                            .onPress(() => ({ type: 'deleteTask' as const, id: task.id }))
                    )
            )
        }

        // Filter buttons
        const filterButton = (label: string, filter: 'all' | 'active' | 'completed') => {
            const isActive = state.filter === filter
            return Button.new(
                Text.new(label).size(isActive ? 14 : 12)
            ).onPress(() => ({ type: 'setFilter' as const, filter }))
        }

        return Container.new(
            Column.new()
                .spacing(16)
                .width(Length.fixed(400))
                .push(Text.new('Task Manager').size(28))
                .push(Rule.horizontal(1))
                // Progress section
                .push(
                    Column.new()
                        .spacing(4)
                        .push(Text.new(`Progress: ${completedCount}/${totalCount} tasks completed`).size(14))
                        .push(ProgressBar.new(0, 1, progress).length(Length.fill()))
                )
                .push(Rule.horizontal(1))
                // Input section
                .push(
                    Row.new()
                        .spacing(10)
                        .push(
                            TextInput.new('Add a new task...', state.inputText)
                                .onInput((value) => ({ type: 'inputChanged' as const, value }))
                                .onSubmit(() => ({ type: 'addTask' }))
                                .width(Length.fill())
                                .padding(Padding.all(8))
                        )
                        .push(
                            Button.new(Text.new('Add'))
                                .onPress(() => ({ type: 'addTask' }))
                                .padding(Padding.xy(16, 8))
                        )
                )
                // Filter buttons
                .push(
                    Row.new()
                        .spacing(8)
                        .push(filterButton('All', 'all'))
                        .push(filterButton('Active', 'active'))
                        .push(filterButton('Completed', 'completed'))
                        .push(Space.new().width(Length.fill()))
                        .push(
                            Button.new(Text.new('Clear Completed'))
                                .onPress(() => ({ type: 'clearCompleted' }))
                        )
                )
                .push(Rule.horizontal(1))
                // Task list in scrollable container
                .push(
                    Scrollable.new(taskList.padding(Padding.all(4)))
                        .height(Length.fixed(300))
                )
                // Empty state message
                .push(
                    filteredTasks.length === 0
                        ? Text.new(
                            state.filter === 'completed' ? 'No completed tasks'
                            : state.filter === 'active' ? 'All tasks completed!'
                            : 'No tasks yet. Add one above!'
                        ).size(14)
                        : Space.new()
                )
        ).center(Length.fill()).padding(Padding.all(20))
    }
})

export const appInstance = { Application }
