// html/cu/tictaetoe/main.js

const X_CLASS = 'x';
const CIRCLE_CLASS = 'circle';
let circleTurn;

const cellElements = document.querySelectorAll('[data-cell]');

startGame();

function startGame() {
    circleTurn = true;
    cellElements.forEach(cell => {
        cell.addEventListener('click', handleClick, { once: true });
    });
    setBoardHoverClass();
}

function handleClick(e) {
    const cell = e.target;
    const currentClass = circleTurn ? CIRCLE_CLASS : X_CLASS;
    placeMark(cell, currentClass);
    if (checkWin(currentClass)) {
        endGame(false);
    } else if (isDraw()) {
        endGame(true);
    } else {
        swapTurns();
        setBoardHoverClass();
    }
}

function endGame(draw) {
    if (draw) {
        alert('게임 끝! 비겼습니다.');
    } else {
        alert(`${circleTurn ? "Circle" : "X"}이(가) 이겼습니다!`);
    }
}

function placeMark(cell, currentClass) {
    cell.classList.add(currentClass);
}

function swapTurns() {
    circleTurn = !circleTurn;
}

function setBoardHoverClass() {
    document.body.classList.remove(X_CLASS);
    document.body.classList.remove(CIRCLE_CLASS);
    if (circleTurn) {
        document.body.classList.add(CIRCLE_CLASS);
    } else {
        document.body.classList.add(X_CLASS);
    }
}

function checkWin(currentClass) {
    return WINNING_COMBINATIONS.some(combination => {
        return combination.every(index => {
            return cellElements[index].classList.contains(currentClass);
        });
    });
}

const WINNING_COMBINATIONS = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6]
];

function isDraw() {
    return [...cellElements].every(cell => {
        return cell.classList.contains(X_CLASS) || cell.classList.contains(CIRCLE_CLASS);
    });
}