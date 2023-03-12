package main

import (
	"fmt"

	"github.com/bytedance/sonic"
	"github.com/gofiber/fiber/v2"
	"github.com/gofiber/fiber/v2/middleware/etag"
)

func main() {
	cfg := GetConfig()

	app := fiber.New(fiber.Config{
		ReduceMemoryUsage: true,
		AppName:           "Backend MalingIT",
		ServerHeader:      "MalingIT",
		JSONEncoder:       sonic.Marshal,
		JSONDecoder:       sonic.Unmarshal,
	})
	app.Use(etag.New())

	app.Get("/", func(c *fiber.Ctx) error {
		c.SendString("MalingIT")
		return nil
	})

	app.Listen(fmt.Sprintf("0.0.0.0:%d", cfg.Port))
}
