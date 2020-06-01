package main

import (
	"fmt"
	"gograph/internal/gograph"
	"log"
	"net/http"
	"os"
)

const port = 8080
const apiPrefix = "/api"

func main() {
	logger := log.New(os.Stdout, "", log.Ldate|log.Ltime)

	logger.Printf("Starting on port=%v\n", port)

	server := http.DefaultServeMux

	gograph.RegisterHandlers(server)

	handler := logRequests(server)

	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", port), handler))
}

func logRequests(handler http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%s %s %s\n", r.RemoteAddr, r.Method, r.URL)
		handler.ServeHTTP(w, r)
	})
}
