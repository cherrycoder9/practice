var board = ['', '', '', '', '', '', '', '', ''];
var currentPlayer = 'O'; // 컴퓨터가 'O', 플레이어가 'X'
var gameActive = true;

function makeMove(cell, index) {
    if (gameActive && board[index] === '' && currentPlayer === 'X') {
        board[index] = currentPlayer;
        cell.textContent = currentPlayer;
        if (checkWin()) {
            alert(currentPlayer + ' wins!');
            gameActive = false;
        } else if (board.indexOf('') === -1) {
            alert('It\'s a draw!');
            gameActive = false;
        } else {
            currentPlayer = 'O';
            computerMove();
        }
    }
}

function computerMove() {
    if (!gameActive) return;

    var emptyCells = [];
    for (var i = 0; i < board.length; i++) {
        if (board[i] === '') {
            emptyCells.push(i);
        }
    }

    var randomIndex = emptyCells[Math.floor(Math.random() * emptyCells.length)];
    board[randomIndex] = currentPlayer;
    document.getElementsByClassName('cell')[randomIndex].textContent = currentPlayer;

    if (checkWin()) {
        alert(currentPlayer + ' wins!');
        gameActive = false;
    } else if (board.indexOf('') === -1) {
        alert('It\'s a draw!');
        gameActive = false;
    } else {
        currentPlayer = 'X';
    }
}

function checkWin() {
    var winConditions = [
        [0, 1, 2], [3, 4, 5], [6, 7, 8],
        [0, 3, 6], [1, 4, 7], [2, 5, 8],
        [0, 4, 8], [2, 4, 6]
    ];

    for (var i = 0; i < winConditions.length; i++) {
        var [a, b, c] = winConditions[i];
        if (board[a] !== '' && board[a] === board[b] && board[a] === board[c]) {
            return true;
        }
    }
    return false;
}

function resetGame() {
    board = ['', '', '', '', '', '', '', '', ''];
    currentPlayer = 'O';
    gameActive = true;
    var cells = document.getElementsByClassName('cell');
    for (var i = 0; i < cells.length; i++) {
        cells[i].textContent = '';
    }
    computerMove(); // 게임 시작 시 컴퓨터가 먼저 둠
}

// 게임 시작 시 컴퓨터가 먼저 둠
window.onload = function () {
    resetGame();
};
