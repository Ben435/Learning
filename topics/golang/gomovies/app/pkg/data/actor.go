package data

import (
	"context"
	"errors"
	"fmt"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
)

type Actor struct {
	ID   string `json:"id" bson:"_id"`
	Name string `json:"name" bson:"name"`
}

type ActorDatasource struct {
	datasource Datasource
}

func NewActorDatasource(datasource Datasource) ActorDatasource {
	return ActorDatasource{
		datasource,
	}
}

func (a *ActorDatasource) GetActor(ctx context.Context, id string) (*Actor, error) {
	actors := a.datasource.GetActorsCollection()

	res := actors.FindOne(ctx, bson.D{{Key: "_id", Value: id}})

	if err := res.Err(); err != nil {
		if err == mongo.ErrNoDocuments {
			return nil, errors.New(fmt.Sprintf("Actor not found with id: %v", id))
		}
		return nil, errors.New(fmt.Sprintf("Failed to find actor with id: %v, err: %v", id, err))
	}

	var actor Actor
	err := res.Decode(&actor)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("Failed to parse actor with id: %v, err: %v", id, err))
	}

	return &actor, nil
}

func (a *ActorDatasource) GetActors(ctx context.Context, ids []string) ([]*Actor, error) {
	actorsCollection := a.datasource.GetActorsCollection()

	cursor, err := actorsCollection.Find(ctx, bson.D{{Key: "_id", Value: bson.D{{Key: "$in", Value: ids}}}})

	if err != nil {
		return nil, errors.New(fmt.Sprintf("Failed to load actors with ids: %v, err: %v", ids, err))
	}

	var actors []*Actor
	for cursor.Next(ctx) {
		var actor Actor
		err := cursor.Decode(&actor)

		if err != nil {
			return nil, errors.New(fmt.Sprintf("Failed to decode actor: %v, err: %v", actor, err))
		}

		actors = append(actors, &actor)
	}

	return actors, nil
}
