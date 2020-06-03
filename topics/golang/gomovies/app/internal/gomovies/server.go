package gomovies

import (
	"gomovies/pkg/data"
	"log"
	"net/http"

	"github.com/graphql-go/handler"
)

func StartServer() {
	server := http.DefaultServeMux

	registerHandlers(server)

	handler := logRequests(server)

	log.Fatal(http.ListenAndServe(":8080", handler))
}

func logRequests(handler http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		log.Printf("%s %s %s\n", r.RemoteAddr, r.Method, r.URL)
		handler.ServeHTTP(w, r)
	})
}

func registerHandlers(server *http.ServeMux) {

	datasource := data.NewDatasource("movies")

	schema := GetSchema(datasource)

	graphqlHandler := handler.New(&handler.Config{
		Schema:   &schema,
		Pretty:   true,
		GraphiQL: true,
	})

	server.Handle("/graphql", graphqlHandler)
}
