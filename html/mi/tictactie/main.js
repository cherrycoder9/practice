const board = document.getElementById('board');
const cells = board.getElementsByTagName('td');

let currentPlayer = 'O'; // computer starts

function makeMove(row, col) {
    if (cells[row * 3 + col].innerHTML === '') {
        cells[row * 3 + col].innerHTML = currentPlayer === 'O' ? 'X' : 'O';
        currentPlayer = currentPlayer === 'O' ? 'X' : 'O';

        if (checkForWin()) {
            setTimeout(() => {
                alert(currentPlayer === 'O' ? 'You win!' : 'Computer wins!');
                resetBoard();
            }, 100);
        } else if (checkForDraw()) {
            setTimeout(() => {
                alert('It\'s a draw!');
                resetBoard();
            }, 100);
        } else {
            // computer's turn
            setTimeout(() => {
                const move = getRandomMove();
                cells[move].innerHTML = 'O';
                currentPlayer = 'X';
            }, 100);
        }
    }
}

function checkForWin() {
    for (let i = 0; i < 3; i++) {
        if (
            cells[i * 3].innerHTML !== '' &&
            cells[i * 3].innerHTML === cells[i * 3 + 1].innerHTML &&
            cells[i * 3 + 1].innerHTML === cells[i * 3 + 2].innerHTML
        ) {
            return true;
        }
    }

    for (let i = 0; i < 3; i++) {
        if (
            cells[i].innerHTML !== '' &&
            cells[i].innerHTML === cells[i + 3].innerHTML &&
            cells[i + 3].innerHTML === cells[i + 6].innerHTML
        ) {
            return true;
        }
    }

    if (
        cells[0].innerHTML !== '' &&
        cells[0].innerHTML === cells[4].innerHTML &&
        cells[4].innerHTML === cells[8].innerHTML
    ) {
        return true;
    }

    if (
        cells[2].innerHTML !== '' &&
        cells[2].innerHTML === cells[4].innerHTML &&
        cells[4].innerHTML === cells[6].innerHTML
    ) {
        return true;
    }

    return false;
}

function checkForDraw() {
    for (let i = 0; i < 9; i++) {
        if (cells[i].innerHTML === '') {
            return false;
        }
    }

    return true;
}

function getRandomMove() {
    let move;
    do {
        move = Math.floor(Math.random() * 9);
    } while (cells[move].innerHTML !== '');
    return move;
}

function resetBoard() {
    for (let i = 0; i < 9; i++) {
        cells[i].innerHTML = '';
    }
    currentPlayer = 'O';
}