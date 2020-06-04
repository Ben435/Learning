package gomovies

import (
	"context"
	"encoding/base64"
	"fmt"
	"gomovies/pkg/data"
	"log"
	"net/http"
	"os"

	"github.com/graphql-go/handler"
)

func StartServer() {
	server := http.DefaultServeMux

	registerHandlers(server)

	handler := logRequests(server)

	port, present := os.LookupEnv("PORT")

	if !present {
		port = "8080"
	}

	fmt.Printf("Starting on port '%v'\n", port)
	log.Fatal(http.ListenAndServe(fmt.Sprintf(":%v", port), handler))
}

func logRequests(handler http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%s %s %s\n", r.RemoteAddr, r.Method, r.URL)
		handler.ServeHTTP(w, r)
	})
}

func registerHandlers(server *http.ServeMux) {
	ctx := context.Background()

	datasource := data.NewMongoDatasource(ctx, "movies")
	encoding := *base64.StdEncoding

	actorsData := data.NewActorDatasource(datasource)
	moviesData := data.NewMovieDatasource(datasource, actorsData, encoding)

	schema := GetSchema(moviesData, actorsData)

	graphqlHandler := handler.New(&handler.Config{
		Schema:   &schema,
		Pretty:   true,
		GraphiQL: true,
	})

	server.Handle("/graphql", graphqlHandler)
}
