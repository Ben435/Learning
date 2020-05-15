package main

import (
	"errors"
	"github.com/urfave/cli/v2"
	"log"
	"os"
)

func main() {
	app := cli.App{
		Name:  "experiment",
		Usage: "experiment <args> <flags>",
		Flags: []cli.Flag {
			&cli.StringFlag{
				Name: "type",
				Value: "basic",
				Usage: "[b]asic|[a]dvanced",
			},
		},
		Action: func(c *cli.Context) error {

			if c.String("type") == "advanced" {
				println("Advanced!")
			} else if c.String("type") == "basic" {
				println("Basic!")
			} else {
				return errors.New("type must be either 'basic' or 'advanced'")
			}

			return nil
		},
	}

	err := (&app).Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}