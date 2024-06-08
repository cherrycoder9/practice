const monthElement = document.getElementById('month');
const daysElement = document.querySelector('.days');
const prevButton = document.getElementById('prev');
const nextButton = document.getElementById('next');
const todoInput = document.getElementById('todo-input');
const addTodoButton = document.getElementById('add-todo');
const todoList = document.getElementById('todo-list');

let currentDate = new Date();

function renderCalendar() {
    const year = currentDate.getFullYear();
    const month = currentDate.getMonth();

    monthElement.innerText = `${year}년 ${month + 1}월`;

    const firstDay = new Date(year, month, 1).getDay();
    const lastDay = new Date(year, month + 1, 0).getDate();

    let dayCount = 1;

    daysElement.innerHTML = '';

    for (let i = 0; i < 6; i++) {
        for (let j = 0; j < 7; j++) {
            if ((i === 0 && j < firstDay) || dayCount > lastDay) {
                daysElement.innerHTML += `<div></div>`;
            } else {
                daysElement.innerHTML += `<div data-date="${year}-${month + 1}-${dayCount}">${dayCount}</div>`;
                dayCount++;
            }
        }
    }

    addEventListenerToDays();
}

function addEventListenerToDays() {
    const days = document.querySelectorAll('.days div');

    days.forEach(day => {
        day.addEventListener('click', (e) => {
            const selectedDate = e.target.dataset.date;
            todoInput.dataset.date = selectedDate;
        });
    });
}

function addTodo() {
    const todoText = todoInput.value.trim();
    const selectedDate = todoInput.dataset.date;

    if (todoText !== '' && selectedDate) {
        const todoItem = {
            id: Date.now(),
            date: selectedDate,
            text: todoText,
        };

        saveTodo(todoItem);
        renderTodoList();

        todoInput.value = '';
        todoInput.dataset.date = '';
    }
}

function saveTodo(todoItem) {
    const todos = JSON.parse(localStorage.getItem('todos')) || [];
    todos.push(todoItem);
    localStorage.setItem('todos', JSON.stringify(todos));
}

function renderTodoList() {
    const selectedDate = todoInput.dataset.date;
    const todos = JSON.parse(localStorage.getItem('todos')) || [];
    const filteredTodos = todos.filter(todo => todo.date === selectedDate);

    todoList.innerHTML = '';

    filteredTodos.forEach(todo => {
        const li = document.createElement('li');
        li.innerText = todo.text;

        const deleteButton = document.createElement('button');
        deleteButton.innerText = '삭제';
        deleteButton.classList.add('delete-todo');
        deleteButton.dataset.id = todo.id;

        li.appendChild(deleteButton);
        todoList.appendChild(li);
    });
}

function deleteTodo(id) {
    const todos = JSON.parse(localStorage.getItem('todos')) || [];
    const updatedTodos = todos.filter(todo => todo.id !== parseInt(id));
    localStorage.setItem('todos', JSON.stringify(updatedTodos));
    renderTodoList();
}

prevButton.addEventListener('click', () => {
    currentDate.setMonth(currentDate.getMonth() - 1);
    renderCalendar();
});

nextButton.addEventListener('click', () => {
    currentDate.setMonth(currentDate.getMonth() + 1);
    renderCalendar();
});

addTodoButton.addEventListener('click', addTodo);

todoList.addEventListener('click', (e) => {
    if (e.target.classList.contains('delete-todo')) {
        const id = e.target.dataset.id;
        deleteTodo(id);
    }
});

renderCalendar();