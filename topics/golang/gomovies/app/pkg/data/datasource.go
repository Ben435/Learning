package data

import (
	"context"
	"fmt"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Datasource interface {
	GetActorsCollection() *mongo.Collection
	GetMoviesCollection() *mongo.Collection
}

type MongoDatasource struct {
	client   mongo.Client
	database string
}

func NewMongoDatasource(ctx context.Context, database string) Datasource {

	fmt.Println("Connecting to MongoDB...")

	clientOptions := options.Client().ApplyURI("mongodb://root:example@database:27017")

	client, err := mongo.Connect(ctx, clientOptions)

	if err != nil {
		log.Fatalf("Error initializing MongoDB connection: %v", err)
	}

	err = client.Ping(ctx, nil)

	if err != nil {
		log.Fatalf("Error pinging established MongoDB connection: %v", err)
	}

	fmt.Println("Connected to MongoDB!")

	datasource := MongoDatasource{
		*client,
		database,
	}

	datasource.loadLocalData(ctx)

	return datasource
}

func (c *MongoDatasource) loadLocalData(ctx context.Context) {
	localData := GetData()
	actorsCollection := c.GetActorsCollection()

	var loadedActors []string
	for _, actor := range localData.Actors {
		res := actorsCollection.FindOne(ctx, bson.D{{Key: "_id", Value: actor.ID}})
		if err := res.Err(); err != nil {
			if err == mongo.ErrNoDocuments {
				_, err := actorsCollection.InsertOne(ctx, actor)

				if err != nil {
					log.Fatalf("Failed to load data for actor: %v, err: %v", actor, err)
				}
				loadedActors = append(loadedActors, actor.ID)
			} else {
				log.Fatalf("Failed to retrieve data about actor: %v, err: %v", actor, err)
			}
		}
	}
	if len(loadedActors) > 0 {
		fmt.Printf("Loaded actors %v\n", loadedActors)
	}

	moviesCollection := c.GetMoviesCollection()
	var loadedMovies []string
	for _, movie := range localData.Movies {
		res := moviesCollection.FindOne(ctx, bson.D{{Key: "_id", Value: movie.ID}})
		if err := res.Err(); err != nil {
			if err == mongo.ErrNoDocuments {
				_, err := moviesCollection.InsertOne(ctx, movie)

				if err != nil {
					log.Fatalf("Failed to load data for movie: %v, err: %v", movie, err)
				}

				loadedMovies = append(loadedMovies, movie.ID)
			} else {
				log.Fatalf("Failed to retrieve data about movie: %v, err: %v", movie, err)
			}
		}
	}

	if len(loadedMovies) > 0 {
		fmt.Printf("Loaded movies %v\n", loadedMovies)
	}
}

func (c MongoDatasource) GetActorsCollection() *mongo.Collection {
	return c.client.Database(c.database).Collection("actors")
}

func (c MongoDatasource) GetMoviesCollection() *mongo.Collection {
	return c.client.Database(c.database).Collection("movies")
}
