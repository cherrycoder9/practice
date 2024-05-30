document.addEventListener('DOMContentLoaded', () => {
    const monthSelector = document.getElementById('monthSelector');
    const calendarBody = document.getElementById('calendarBody');
    const selectedDateElem = document.getElementById('selectedDate');
    const todoInput = document.getElementById('todoInput');
    const colorPicker = document.getElementById('colorPicker');
    const addTodoButton = document.getElementById('addTodoButton');
    const todoList = document.getElementById('todoList');
    let selectedDate = null;

    const months = [
        'January', 'February', 'March', 'April', 'May', 'June',
        'July', 'August', 'September', 'October', 'November', 'December'
    ];

    months.forEach((month, index) => {
        const option = document.createElement('option');
        option.value = index;
        option.textContent = month;
        monthSelector.appendChild(option);
    });

    monthSelector.addEventListener('change', (e) => {
        renderCalendar(e.target.value);
    });

    function renderCalendar(month) {
        calendarBody.innerHTML = '';
        const date = new Date(2024, month, 1);
        const firstDay = date.getDay();
        const lastDate = new Date(2024, month + 1, 0).getDate();

        for (let i = 0; i < firstDay; i++) {
            const emptyDiv = document.createElement('div');
            calendarBody.appendChild(emptyDiv);
        }

        for (let i = 1; i <= lastDate; i++) {
            const dayDiv = document.createElement('div');
            dayDiv.className = 'calendar-day';
            dayDiv.textContent = i;
            dayDiv.addEventListener('click', () => {
                selectedDate = `${2024}-${month + 1}-${i}`;
                selectedDateElem.textContent = selectedDate;
                renderTodoList();
            });
            calendarBody.appendChild(dayDiv);
        }
    }

    addTodoButton.addEventListener('click', () => {
        if (selectedDate && todoInput.value) {
            const todos = JSON.parse(localStorage.getItem(selectedDate)) || [];
            todos.push({ text: todoInput.value, color: colorPicker.value });
            localStorage.setItem(selectedDate, JSON.stringify(todos));
            todoInput.value = '';
            renderTodoList();
        }
    });

    function renderTodoList() {
        if (!selectedDate) return;

        todoList.innerHTML = '';
        const todos = JSON.parse(localStorage.getItem(selectedDate)) || [];
        todos.forEach((todo, index) => {
            const todoItem = document.createElement('li');
            todoItem.className = 'todo-item';
            todoItem.style.backgroundColor = todo.color;

            const span = document.createElement('span');
            span.textContent = todo.text;

            const deleteButton = document.createElement('button');
            deleteButton.textContent = '삭제';
            deleteButton.addEventListener('click', () => {
                todos.splice(index, 1);
                localStorage.setItem(selectedDate, JSON.stringify(todos));
                renderTodoList();
            });

            todoItem.appendChild(span);
            todoItem.appendChild(deleteButton);
            todoList.appendChild(todoItem);
        });
    }

    monthSelector.value = new Date().getMonth();
    renderCalendar(new Date().getMonth());
});
