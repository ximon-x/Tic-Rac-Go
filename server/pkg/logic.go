package pkg

import (
	"crypto/rand"
	"math/big"
)

func checkWin(board [][]int) bool {
	n := len(board)

	// Check Rows
	for _, row := range board {
		if row[0] == 0 {
			continue
		}
		won := true

		for _, cell := range row {
			if row[0] != cell {
				won = false
				break
			}
		}

		if won {
			return true
		}

	}

	// Check Columns
	for i := 0; i < n; i++ {
		if board[0][i] == 0 {
			continue
		}
		won := true

		for j := 0; j < n; j++ {
			if board[0][i] != board[j][i] {
				won = false
				break
			}
		}

		if won {
			return true
		}
	}

	// Check Diagonals
	if board[0][0] == 0 && board[0][n-1] == 0 {
		return false
	}

	right_won := true
	left_won := true

	for i := 0; i < n; i++ {
		if board[0][0] != board[i][i] && board[0][0] != 0 {
			right_won = false
		}

		if board[0][n-1] != board[i][n-1-i] && board[0][n-1] != 0 {
			left_won = false
		}
	}

	if right_won && board[0][0] != 0 || left_won && board[0][n-1] != 0 {
		return true
	} else {
		return false
	}
}

func makeMove(board *[][]int, player string) ([]int, error) {
	var NoOpenSpotsError error
	var openSpots [][]int

	max := big.NewInt(100000000)
	randInt, err := rand.Int(rand.Reader, max)
	if err != nil {
		return nil, NoOpenSpotsError
	}

	for i, row := range *board {
		for j, cell := range row {
			if cell == 0 {
				openSpots = append(openSpots, []int{i, j})
			}
		}
	}

	if len(openSpots) == 0 {
		return nil, NoOpenSpotsError
	}

	move := openSpots[randInt.Int64()%int64(len(openSpots))]

	if player == "X" {
		(*board)[move[0]][move[1]] = 2
	} else {
		(*board)[move[0]][move[1]] = 1
	}

	return move, nil
}
