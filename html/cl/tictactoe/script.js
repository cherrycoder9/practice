var board = [
    ['', '', ''],
    ['', '', ''],
    ['', '', '']
];
var humanPlayer = 'O';
var computerPlayer = 'X';
var currentPlayer = computerPlayer;

function makeMove(row, col) {
    if (currentPlayer === humanPlayer && board[row][col] === '') {
        board[row][col] = humanPlayer;
        document.getElementById('cell-' + row + '-' + col).innerHTML = humanPlayer;
        currentPlayer = computerPlayer;
        if (!checkWinner() && !checkDraw()) {
            setTimeout(makeComputerMove, 500);
        }
    }
}

function makeComputerMove() {
    var emptyCells = [];
    for (var i = 0; i < 3; i++) {
        for (var j = 0; j < 3; j++) {
            if (board[i][j] === '') {
                emptyCells.push([i, j]);
            }
        }
    }
    var randomIndex = Math.floor(Math.random() * emptyCells.length);
    var [row, col] = emptyCells[randomIndex];
    board[row][col] = computerPlayer;
    document.getElementById('cell-' + row + '-' + col).innerHTML = computerPlayer;
    currentPlayer = humanPlayer;
    if (checkWinner()) {
        alert(computerPlayer + '가 이겼습니다!');
        resetGame();
    } else if (checkDraw()) {
        alert('무승부입니다!');
        resetGame();
    }
}

function checkWinner() {
    // 가로 확인
    for (var i = 0; i < 3; i++) {
        if (board[i][0] !== '' && board[i][0] === board[i][1] && board[i][1] === board[i][2]) {
            return true;
        }
    }
    // 세로 확인
    for (var j = 0; j < 3; j++) {
        if (board[0][j] !== '' && board[0][j] === board[1][j] && board[1][j] === board[2][j]) {
            return true;
        }
    }
    // 대각선 확인
    if (board[0][0] !== '' && board[0][0] === board[1][1] && board[1][1] === board[2][2]) {
        return true;
    }
    if (board[0][2] !== '' && board[0][2] === board[1][1] && board[1][1] === board[2][0]) {
        return true;
    }
    return false;
}

function checkDraw() {
    for (var i = 0; i < 3; i++) {
        for (var j = 0; j < 3; j++) {
            if (board[i][j] === '') {
                return false;
            }
        }
    }
    return true;
}

function resetGame() {
    board = [
        ['', '', ''],
        ['', '', ''],
        ['', '', '']
    ];
    currentPlayer = computerPlayer;
    var cells = document.getElementsByTagName('td');
    for (var i = 0; i < cells.length; i++) {
        cells[i].innerHTML = '';
    }
    setTimeout(makeComputerMove, 500);
}

// 게임 시작 시 컴퓨터 먼저 시작
setTimeout(makeComputerMove, 500);