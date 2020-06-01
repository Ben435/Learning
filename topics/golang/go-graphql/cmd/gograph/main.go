package main

import (
	"fmt"
	"gograph/internal/gograph"
	"log"
	"net/http"
)

const rootURL = "http://localhost"
const port = 8080
const apiPrefix = "/api"

func getHostURL() string {
	return fmt.Sprintf("%s:%d", rootURL, port)
}

func main() {
	gograph.RegisterHandlers()
	log.Fatal(http.ListenAndServe(":8080", nil))
}
