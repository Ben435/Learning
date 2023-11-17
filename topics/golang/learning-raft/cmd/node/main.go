package main

import (
	"fmt"
	"log/slog"
	"math/rand"
	"net/http"
	"time"

	"github.com/caarlos0/env/v10"
)

type Config struct {
	NodeID       int      `env:"NODE_ID" envDefault:"0"`
	AllNodePorts []string `env:"ALL_NODE_PORTS" envSeparator:","`
}

type State struct {
	shouldStart           bool
	hasStarted            bool
	termCounter           int
	isLeader              bool
	lastContactedByLeader time.Time
}

func main() {
	cfg := Config{}
	err := env.Parse(&cfg)
	if err != nil {
		panic(err)
	}

	port := cfg.AllNodePorts[cfg.NodeID]

	slog.Info("starting node %d on port %d", cfg.NodeID, port)

	state := State{
		shouldStart:           false,
		hasStarted:            false,
		termCounter:           0,
		isLeader:              false,
		lastContactedByLeader: time.Now(),
	}

	mux := http.NewServeMux()

	mux.HandleFunc("/heartbeat", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		state.lastContactedByLeader = time.Now()
	}))

	mux.HandleFunc("/force-start-election", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {

	}))

	go http.ListenAndServe(fmt.Sprintf(":%s", port), mux)

	for {
		if state.shouldStart && !state.hasStarted {
			state.hasStarted = true
			time.Sleep(150*time.Millisecond + time.Millisecond*time.Duration(rand.Intn(150)))
		}
	}
}
