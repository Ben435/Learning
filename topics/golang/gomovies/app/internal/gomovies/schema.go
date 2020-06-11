package gomovies

import (
	"fmt"
	"gomovies/pkg/data"
	"log"

	"github.com/graphql-go/graphql"
)

func GetSchema(movieDatasource data.MovieDatasource, actorDatasource data.ActorDatasource) graphql.Schema {

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
						return movieDatasource.GetActorsForMovie(p.Context, *movie)
					}
					return nil, nil
				},
			},
		},
	})

	pageInfoType := graphql.NewObject(graphql.ObjectConfig{
		Name: "PageInfo",
		Fields: graphql.Fields{
			"hasNextPage": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if pageInfo, ok := p.Source.(*data.PageInfo); ok {
						return pageInfo.HasNextPage, nil
					}
					return nil, nil
				},
			},
			"hasPreviousPage": &graphql.Field{
				Type: graphql.NewNonNull(graphql.Boolean),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if pageInfo, ok := p.Source.(*data.PageInfo); ok {
						return pageInfo.HasPreviousPage, nil
					}
					return nil, nil
				},
			},
			"startCursor": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if pageInfo, ok := p.Source.(*data.PageInfo); ok {
						return pageInfo.StartCursor, nil
					}
					return nil, nil
				},
			},
			"endCursor": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if pageInfo, ok := p.Source.(*data.PageInfo); ok {
						return pageInfo.EndCursor, nil
					}
					return nil, nil
				},
			},
		},
	})

	movieNodeType := graphql.NewObject(graphql.ObjectConfig{
		Name: "Node",
		Fields: graphql.Fields{
			"node": &graphql.Field{
				Type: movieType,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if movieNode, ok := p.Source.(*data.MovieNode); ok {
						return &movieNode.Data, nil
					}
					return nil, nil
				},
			},
			"cursor": &graphql.Field{
				Type: graphql.NewNonNull(graphql.String),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if movieNode, ok := p.Source.(*data.MovieNode); ok {
						return movieNode.Cursor, nil
					}
					return nil, nil
				},
			},
		},
	})

	movieConnectionType := graphql.NewObject(graphql.ObjectConfig{
		Name: "MovieConnection",
		Fields: graphql.Fields{
			"pageInfo": &graphql.Field{
				Type: graphql.NewNonNull(pageInfoType),
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if movieConnection, ok := p.Source.(*data.MovieConnection); ok {
						return &movieConnection.PageInfo, nil
					}
					return nil, nil
				},
			},
			"edges": &graphql.Field{
				Type: &graphql.List{
					OfType: movieNodeType,
				},
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					if movieConnection, ok := p.Source.(*data.MovieConnection); ok {
						return movieConnection.Edges, nil
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
					return actorDatasource.GetActor(p.Context, id)
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

					return movieDatasource.GetMovie(p.Context, id)
				},
			},
			"movies": &graphql.Field{
				Type: movieConnectionType,
				Resolve: func(p graphql.ResolveParams) (interface{}, error) {
					return movieDatasource.GetMovies(p.Context)
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
