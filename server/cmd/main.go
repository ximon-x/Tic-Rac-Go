package main

import (
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/cors"
	"github.com/gofiber/fiber/v2/middleware/logger"
	"github.com/ximon-x/Tic-Tac-Go/server/pkg"
)

func main() {
	// Create Go Fiber app
	app := fiber.New()

	app.Use(cors.New(cors.Config{
		AllowOrigins: "*",
		AllowHeaders: "Origin, Content-Type, Accept",
	}))

	app.Use(logger.New())

	app.Get("/", func(c *fiber.Ctx) error {
		return c.JSON(
			fiber.Map{
				"health": "ok",
			},
		)
	})

	app.Post("/play", pkg.Play)

	// Start server on http://localhost:3000
	app.Listen(":3000")
}
