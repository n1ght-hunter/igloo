"""Example Python plugin for Igloo - Task Manager application."""

from dataclasses import dataclass
from typing import Literal

from igloo_py import (
    App,
    igloo_app,
    Text,
    Column,
    Row,
    Button,
    Container,
    TextInput,
    Checkbox,
    ProgressBar,
    Rule,
    Space,
    Scrollable,
    Length,
    Padding,
    MessageManager,
    ElementLike,
    Message as WitMessage,
)


# Task type
@dataclass
class Task:
    id: int
    text: str
    completed: bool


FilterType = Literal["all", "active", "completed"]


# Messages
@dataclass
class InputChanged:
    value: str


@dataclass
class AddTask:
    pass


@dataclass
class ToggleTask:
    id: int


@dataclass
class DeleteTask:
    id: int


@dataclass
class SetFilter:
    filter: FilterType


@dataclass
class ClearCompleted:
    pass


Msg = InputChanged | AddTask | ToggleTask | DeleteTask | SetFilter | ClearCompleted


def get_string(msg: WitMessage) -> str | None:
    """Extract string from message."""
    if isinstance(msg, dict) and msg.get("tag") == "string-type":
        return msg.get("val")
    return None


@igloo_app
class TaskManagerApp(App[Msg]):
    """Task Manager application."""

    def __init__(self):
        self.tasks: list[Task] = [
            Task(1, "Learn igloo-py", True),
            Task(2, "Build a cool app", False),
            Task(3, "Share with others", False),
        ]
        self.input_text = ""
        self.next_id = 4
        self.filter: FilterType = "all"

    def update(self, msg: Msg) -> None:
        match msg:
            case InputChanged(value=value):
                self.input_text = value

            case AddTask():
                if self.input_text.strip():
                    self.tasks.append(Task(self.next_id, self.input_text.strip(), False))
                    self.input_text = ""
                    self.next_id += 1

            case ToggleTask(id=task_id):
                for task in self.tasks:
                    if task.id == task_id:
                        task.completed = not task.completed
                        break

            case DeleteTask(id=task_id):
                self.tasks = [t for t in self.tasks if t.id != task_id]

            case SetFilter(filter=f):
                self.filter = f

            case ClearCompleted():
                self.tasks = [t for t in self.tasks if not t.completed]

    def view(self, messages: MessageManager[Msg]) -> ElementLike:
        completed_count = sum(1 for t in self.tasks if t.completed)
        total_count = len(self.tasks)
        progress = completed_count / total_count if total_count > 0 else 0.0

        # Filter tasks
        if self.filter == "active":
            filtered_tasks = [t for t in self.tasks if not t.completed]
        elif self.filter == "completed":
            filtered_tasks = [t for t in self.tasks if t.completed]
        else:
            filtered_tasks = self.tasks

        # Build task list
        task_list = Column.new().spacing(8)
        for task in filtered_tasks:
            task_id = task.id  # Capture for closure
            task_list.push(
                Row.new()
                .spacing(10)
                .push(
                    Checkbox.new(task.completed)
                    .label(task.text)
                    .on_toggle(messages, lambda tid=task_id: ToggleTask(tid))
                )
                .push(Space.new().width(Length.fill()))
                .push(
                    Button.new(Text.new("×").size(16)).on_press(
                        messages, lambda tid=task_id: DeleteTask(tid)
                    )
                )
            )

        # Filter buttons helper
        def filter_button(label: str, f: FilterType) -> ElementLike:
            is_active = self.filter == f
            return Button.new(Text.new(label).size(14 if is_active else 12)).on_press(
                messages, lambda filter_val=f: SetFilter(filter_val)
            )

        # Empty state message
        if not filtered_tasks:
            if self.filter == "completed":
                empty_msg = "No completed tasks"
            elif self.filter == "active":
                empty_msg = "All tasks completed!"
            else:
                empty_msg = "No tasks yet. Add one above!"
            empty_widget: ElementLike = Text.new(empty_msg).size(14)
        else:
            empty_widget = Space.new()

        return (
            Container.new(
                Column.new()
                .spacing(16)
                .width(Length.fixed(400))
                .push(Text.new("Task Manager").size(28))
                .push(Rule.horizontal(1))
                # Progress section
                .push(
                    Column.new()
                    .spacing(4)
                    .push(
                        Text.new(f"Progress: {completed_count}/{total_count} tasks completed").size(
                            14
                        )
                    )
                    .push(ProgressBar.new(0, 1, progress).length(Length.fill()))
                )
                .push(Rule.horizontal(1))
                # Input section
                .push(
                    Row.new()
                    .spacing(10)
                    .push(
                        TextInput.new("Add a new task...", self.input_text)
                        .on_input(messages, lambda m: InputChanged(get_string(m) or ""))
                        .on_submit(messages, lambda: AddTask())
                        .width(Length.fill())
                        .padding(Padding.all(8))
                    )
                    .push(
                        Button.new(Text.new("Add"))
                        .on_press(messages, lambda: AddTask())
                        .padding(Padding.xy(16, 8))
                    )
                )
                # Filter buttons
                .push(
                    Row.new()
                    .spacing(8)
                    .push(filter_button("All", "all"))
                    .push(filter_button("Active", "active"))
                    .push(filter_button("Completed", "completed"))
                    .push(Space.new().width(Length.fill()))
                    .push(
                        Button.new(Text.new("Clear Completed")).on_press(
                            messages, lambda: ClearCompleted()
                        )
                    )
                )
                .push(Rule.horizontal(1))
                # Task list in scrollable container
                .push(Scrollable.new(task_list.padding(Padding.all(4))).height(Length.fixed(300)))
                # Empty state message
                .push(empty_widget)
            )
            .center(Length.fill())
            .padding(Padding.all(20))
        )


# WitWorld and Message are automatically exported by @igloo_app decorator
