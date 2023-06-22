package pkg

import (
	"math/rand"
	"time"

	"github.com/gofiber/fiber/v2"
)

func makeMove(board *[][]int, player string) ([]int, error) {
	// Do not use math/rand in production, it is not cryptographically secure.
	// Use crypto/rand instead.
	rand.Seed(time.Now().UnixNano())

	var NoOpenSpotsError error
	var openSpots [][]int

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

	move := openSpots[rand.Intn(len(openSpots))]

	if player == "X" {
		(*board)[move[0]][move[1]] = 2
	} else {
		(*board)[move[0]][move[1]] = 1
	}

	return move, nil
}

func Play(c *fiber.Ctx) error {
	var game Game

	if err := c.BodyParser(&game); err != nil {
		return err
	}

	if checkWin(game.Board) {
		game.GameOver = true
		return c.JSON(
			fiber.Map{
				"game_over": game.GameOver,
				"move":      nil,
			},
		)
	}

	move, err := makeMove(&game.Board, game.Player)
	if err != nil {
		game.GameOver = true
		return c.JSON(
			fiber.Map{
				"game_over": game.GameOver,
				"move":      nil,
			},
		)
	}

	if checkWin(game.Board) {
		game.GameOver = true
		return c.JSON(
			fiber.Map{
				"game_over": game.GameOver,
				"move":      nil,
			},
		)
	}

	return c.JSON(
		fiber.Map{
			"game_over": game.GameOver,
			"move":      move,
		},
	)
}
