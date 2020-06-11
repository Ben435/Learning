package data

import (
	"context"
	"encoding/base64"
	"fmt"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

type Movie struct {
	ID       string   `json:"id" bson:"_id"`
	Title    string   `json:"title" bson:"title"`
	ActorIDs []string `json:"actors" bson:"actor_ids"`
}

type MovieNode struct {
	Data   Movie
	Cursor string
}

type MovieConnection struct {
	Edges    []*MovieNode
	PageInfo PageInfo
}

type MovieDatasource struct {
	datasource      Datasource
	actorDatasource ActorDatasource
	encoding        base64.Encoding
}

func NewMovieDatasource(datasource Datasource, actorDatasource ActorDatasource, encoding base64.Encoding) MovieDatasource {
	return MovieDatasource{
		datasource,
		actorDatasource,
		encoding,
	}
}

func (m *MovieDatasource) GetActorsForMovie(ctx context.Context, movie Movie) ([]*Actor, error) {
	return m.actorDatasource.GetActors(ctx, movie.ActorIDs)
}

func (m *MovieDatasource) GetMovie(ctx context.Context, id string) (*Movie, error) {
	movies := m.datasource.GetMoviesCollection()

	res := movies.FindOne(ctx, bson.D{{Key: "_id", Value: id}})

	if err := res.Err(); err != nil {
		if err == mongo.ErrNoDocuments {
			return nil, fmt.Errorf("Movie not found with id: %v", id)
		}
		return nil, fmt.Errorf("Failed to find movie with id: %v, err: %v", id, err)
	}

	var movie Movie
	res.Decode(&movie)

	return &movie, nil
}

func (m *MovieDatasource) GetMovies(ctx context.Context) (*MovieConnection, error) {
	moviesCollection := m.datasource.GetMoviesCollection()

	cursor, err := moviesCollection.Find(ctx, bson.D{})

	if err != nil {
		return nil, fmt.Errorf("Error loading movies: %v", err)
	} else if err := cursor.Err(); err != nil {
		return nil, fmt.Errorf("Cursor error loading movies: %v", err)
	}

	var movies []*MovieNode
	for cursor.Next(ctx) {
		var movie Movie
		err := cursor.Decode(&movie)
		if err != nil {
			return nil, fmt.Errorf("Failed to decode data: %v, err: %v", cursor.Current, err)
		}

		movieNode := MovieNode{
			Data:   movie,
			Cursor: m.encoding.EncodeToString([]byte(movie.ID)),
		}

		movies = append(movies, &movieNode)
	}

	pageInfo := PageInfo{
		HasNextPage:     false,
		HasPreviousPage: false,
	}

	if len(movies) > 0 {
		pageInfo.StartCursor = movies[0].Cursor
		pageInfo.EndCursor = movies[len(movies)-1].Cursor
	}

	return &MovieConnection{
		PageInfo: pageInfo,
		Edges:    movies,
	}, nil
}
