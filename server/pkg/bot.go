package pkg

import (
	"crypto/rand"
	"math/big"

	"github.com/gofiber/fiber/v2"
)

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
