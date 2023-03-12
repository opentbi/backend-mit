package main

import (
	"fmt"
	"os"

	"github.com/bytedance/sonic"
)

type Config struct {
	AppID   string `json:"app_id"`
	AppHash string `json:"app_hash"`

	Channel string `json:"telegram_channel"`
	Port    int    `json:"port"`
}

func GetConfig() *Config {
	cfg := &Config{}

	bytes, err := os.ReadFile("config.json")
	if err != nil {
		panic(err)
	}

	if err = sonic.Unmarshal(bytes, cfg); err != nil {
		panic(err)
	}

	if len(fmt.Sprint(cfg.Port)) != 4 {
		panic("Port should be 4 character")
	}

	return cfg
}
