package data

import (
	"context"
	"errors"
	"fmt"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

type Movie struct {
	ID       string   `json:"id" bson:"_id"`
	Title    string   `json:"title" bson:"title"`
	ActorIDs []string `json:"actors" bson:"actor_ids"`
}

type MovieDatasource struct {
	datasource      Datasource
	actorDatasource ActorDatasource
}

func NewMovieDatasource(datasource Datasource, actorDatasource ActorDatasource) MovieDatasource {
	return MovieDatasource{
		datasource,
		actorDatasource,
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
			return nil, errors.New(fmt.Sprintf("Movie not found with id: %v", id))
		}
		return nil, errors.New(fmt.Sprintf("Failed to find movie with id: %v, err: %v", id, err))
	}

	var movie Movie
	res.Decode(&movie)

	return &movie, nil
}
