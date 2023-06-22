package pkg

type Game struct {
	Board    [][]int `json:"board"`
	Player   string  `json:"player"`
	GameOver bool    `json:"game_over"`
}

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
