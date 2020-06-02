package gograph

import (
	"fmt"
	"log"

	"github.com/graphql-go/graphql"
)

type Character struct {
	ID   string `json:"id"`
	Name string `json:"name"`
	Age  int    `json:"age"`
}

// GetSchema returns GraphQL schema
func GetSchema() graphql.Schema {

	characterType := graphql.NewObject(graphql.ObjectConfig{
		Name:        "Character",
		Description: "Character in a movie",
		Fields: graphql.Fields{
			"id": &graphql.Field{
				Type: graphql.String,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if character, ok := p.Source.(Character); ok {
						return character.ID, nil
					}
					return nil, nil
				},
			},
			"name": &graphql.Field{
				Type: graphql.String,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if character, ok := p.Source.(Character); ok {
						return character.Name, nil
					}
					return nil, nil
				},
			},
			"age": &graphql.Field{
				Type: graphql.Int,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if character, ok := p.Source.(Character); ok {
						return character.Age, nil
					}
					return nil, nil
				},
			},
		},
	})

	rootQuery := graphql.NewObject(graphql.ObjectConfig{
		Name: "RootQuery",
		Fields: graphql.Fields{
			"character": &graphql.Field{
				Type: characterType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Description: "ID of character",
						Type:        graphql.NewNonNull(graphql.String),
					},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					var id string = fmt.Sprintf("%v", p.Args["id"])
					return Character{
						ID:   id,
						Name: "bob",
						Age:  3,
					}, nil
				},
			},
		},
	})

	schemaConfig := graphql.SchemaConfig{Query: rootQuery}
	schema, err := graphql.NewSchema(schemaConfig)

	if err != nil {
		log.Fatalf("Error creating Graphql schema: '%v'", err)
	}

	return schema
}
