var board = ['', '', '', '', '', '', '', '', ''];
var currentPlayer = 'X';
var gameOver = false;

var cells = document.querySelectorAll('td');
var resultDiv = document.getElementById('result');

// 컴퓨터가 먼저 시작 
window.onload = function () {
    // 컴퓨터의 첫 턴
    var firstTurnIndex = computerTurn();
    handleTurn(firstTurnIndex);
};

// 셀 클릭 이벤트 처리
function handleClick(index) {
    // 사용자 턴일 때만 처리
    if (gameOver || board[index] !== '' || currentPlayer === 'O') {
        return;
    }
    handleTurn(index);
}

function handleTurn(index) {
    // index가 유효한지 확인 (computerTurn()에서 빈 셀이 없을 경우 -1을 반환)
    if (index === -1) {
        return;
    }

    board[index] = currentPlayer;
    cells[index].innerText = currentPlayer;
    cells[index].classList.add("disabled");

    if (checkWin(currentPlayer)) {
        endGame(currentPlayer + " 승리!");
    } else if (checkTie()) {
        endGame("무승부!");
    } else {
        // *** 플레이어 전환 (수정된 위치) *** 
        currentPlayer = (currentPlayer === 'X') ? 'O' : 'X';

        // 컴퓨터 턴일 경우에만 computerTurn() 호출
        if (currentPlayer === 'O') {
            var computerIndex = computerTurn();
            handleTurn(computerIndex);
        }
    }
}

function computerTurn() {
    // 간단한 AI: 랜덤하게 빈 셀 선택
    var availableCells = [];
    for (var i = 0; i < board.length; i++) {
        if (board[i] === '') {
            availableCells.push(i);
        }
    }

    // 빈 셀이 없는 경우 -1 반환
    if (availableCells.length === 0) {
        return -1;
    }

    var randomIndex = availableCells[Math.floor(Math.random() * availableCells.length)];

    // setTimeout 제거, 바로 index 반환
    return randomIndex;
}

function checkWin(player) {
    // 가로 체크
    for (var i = 0; i < 3; i++) {
        if (
            board[i * 3] === player &&
            board[i * 3 + 1] === player &&
            board[i * 3 + 2] === player
        ) {
            return true;
        }
    }

    // 세로 체크
    for (var j = 0; j < 3; j++) {
        if (
            board[j] === player &&
            board[j + 3] === player &&
            board[j + 6] === player
        ) {
            return true;
        }
    }

    // 대각선 체크
    if (
        (board[0] === player && board[4] === player && board[8] === player) ||
        (board[2] === player && board[4] === player && board[6] === player)
    ) {
        return true;
    }

    return false;
}

function checkTie() {
    for (var i = 0; i < board.length; i++) {
        if (board[i] === '') {
            return false;
        }
    }
    return true;
}

function highlightWinningCells(player) {
    for (var i = 0; i < 9; i++) {
        if (board[i] === player) {
            cells[i].classList.add("end");
        }
    }
}

function endGame(message) {
    resultDiv.innerText = message;
    gameOver = true;
    highlightWinningCells(currentPlayer);
}