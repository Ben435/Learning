package main

import (
	"fmt"
	"gograph/internal/gograph"
	"log"
	"net/http"
	"os"
)

const rootURL = "http://localhost"
const port = 8080
const apiPrefix = "/api"

func getHostURL() string {
	return fmt.Sprintf("%s:%d", rootURL, port)
}

func main() {
	logger := log.New(os.Stdout, "", log.Ldate|log.Ltime)

	logger.Printf("Starting on %v\n", getHostURL())
	gograph.RegisterHandlers()
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%d", port), nil))
}
