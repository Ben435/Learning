package main

import (
	"fmt"
	"log/slog"
	"net/http"
	"os"

	"github.com/go-chi/chi"
)

func main() {
	port := os.Getenv("PORT")
	if len(port) <= 0 {
		port = "3000"
	}

	logger := slog.New(slog.NewJSONHandler(os.Stdout, nil))

	r := chi.NewRouter()
	r.Get("/", func(w http.ResponseWriter, r *http.Request) {
		logger.Info("received request")
		w.Write([]byte("hello!\n"))
	})

	logger.With("port", port).Info("starting service")
	http.ListenAndServe(fmt.Sprintf(":%s", port), r)
}
