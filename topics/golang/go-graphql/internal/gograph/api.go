package gograph

import (
	"net/http"

	"github.com/graphql-go/handler"
)

// RegisterHandlers registers handlers to target
func RegisterHandlers() {
	schema := GetSchema()

	graphqlHandler := handler.New(&handler.Config{
		Schema:   &schema,
		Pretty:   true,
		GraphiQL: GetBoolConfigVar(DebugMode),
	})

	http.Handle("/graphql", graphqlHandler)
}
