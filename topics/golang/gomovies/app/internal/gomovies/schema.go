package gomovies

import (
	"fmt"
	"gomovies/pkg/data"
	"log"

	"github.com/graphql-go/graphql"
)

func GetSchema(datasource data.Datasource) graphql.Schema {

	actorType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Actor",
		Fields: graphql.Fields{
			"id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if actor, ok := p.Source.(*data.Actor); ok {
						return actor.ID, nil
					}
					return nil, nil
				},
			},
			"name": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if actor, ok := p.Source.(*data.Actor); ok {
						return actor.Name, nil
					}
					return nil, nil
				},
			},
		},
	})

	movieType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Movie",
		Fields: graphql.Fields{
			"id": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if movie, ok := p.Source.(*data.Movie); ok {
						return movie.ID, nil
					}
					return nil, nil
				},
			},
			"title": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if movie, ok := p.Source.(*data.Movie); ok {
						return movie.Title, nil
					}
					return nil, nil
				},
			},
			"actors": &graphql.Field{
				Type: &graphql.List{
					OfType: actorType,
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if movie, ok := p.Source.(*data.Movie); ok {
						return movie.GetActors(datasource)
					}
					return nil, nil
				},
			},
		},
	})

	rootQuery := graphql.NewObject(graphql.ObjectConfig{
		Name: "RootQuery",
		Fields: graphql.Fields{
			"actor": &graphql.Field{
				Type: actorType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Type: graphql.NewNonNull(graphql.String),
					},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := fmt.Sprintf("%v", p.Args["id"])
					return data.GetActor(datasource, id)
				},
			},
			"movie": &graphql.Field{
				Type: movieType,
				Args: graphql.FieldConfigArgument{
					"id": &graphql.ArgumentConfig{
						Type: graphql.NewNonNull(graphql.String),
					},
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					id := fmt.Sprintf("%v", p.Args["id"])
					return data.GetMovie(datasource, id)
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
