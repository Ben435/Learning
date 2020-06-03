package data

import (
	"context"
	"errors"
	"fmt"

	"go.mongodb.org/mongo-driver/bson"
)

type Actor struct {
	ID   string `json:"id" bson:"_id"`
	Name string `json:"name" bson:"name"`
}

func GetActor(datasource Datasource, id string) (*Actor, error) {
	actors := datasource.GetActorsCollection()

	res := actors.FindOne(context.TODO(), bson.D{{Key: "_id", Value: id}})

	if err := res.Err(); err != nil {
		return nil, errors.New(fmt.Sprintf("Failed to find actor with id: %v, err: %v", id, err))
	}

	var actor Actor
	err := res.Decode(&actor)
	if err != nil {
		return nil, errors.New(fmt.Sprintf("Failed to parse actor with id: %v, err: %v", id, err))
	}

	return &actor, nil
}
