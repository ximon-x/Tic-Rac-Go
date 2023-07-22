package pkg

import (
	"github.com/gofiber/fiber/v2"
)

type Game struct {
	Board    [][]int `json:"board"`
	Player   string  `json:"player"`
	GameOver bool    `json:"game_over"`
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
