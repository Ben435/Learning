package graphql

import (
	"log"

	"github.com/graphql-go/graphql"
)

func GetSchema() graphql.Schema {

	rootQuery := graphql.NewObject(graphql.ObjectConfig{
		Name: "RootQuery",
		Fields: graphql.Fields{
			"actor",
		}
	})

	schemaConfig := graphql.SchemaConfig{Query: rootQuery}
	schema, err := graphql.NewSchema(schemaConfig)

	if err != nil {
		log.Fatalf("Error creating Graphql schema: '%v'", err)
	}

	return schema
}
