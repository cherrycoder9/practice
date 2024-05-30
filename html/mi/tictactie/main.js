var board = ['', '', '', '', '', '', '', '', ''];
var currentPlayer = 'X';
var gameOver = false;

function play(index) {
    if (board[index] === '' && !gameOver) {
        board[index] = currentPlayer;
        document.getElementById('cell-' + index).innerText = currentPlayer;
        if (checkWin(currentPlayer)) {
            document.getElementById('status').innerText = currentPlayer + ' wins!';
            gameOver = true;
        } else if (board.every(cell => cell !== '')) {
            document.getElementById('status').innerText = 'It\'s a draw!';
            gameOver = true;
        } else {
            currentPlayer = currentPlayer === 'X' ? 'O' : 'X';
        }
    }
}

function checkWin(player) {
    var winConditions = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6]
    ];
    return winConditions.some(condition => {
        return condition.every(index => {
            return board[index] === player;
        });
    });
}

function reset() {
    board = ['', '', '', '', '', '', '', '', ''];
    currentPlayer = 'X';
    gameOver = false;
    document.getElementById('status').innerText = '';
    for (var i = 0; i < 9; i++) {
        document.getElementById('cell-' + i).innerText = '';
    }
}
