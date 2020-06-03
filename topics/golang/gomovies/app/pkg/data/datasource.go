package data

import (
	"context"
	"fmt"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
)

type Datasource struct {
	client   mongo.Client
	database string
}

func NewDatasource(database string) Datasource {

	fmt.Println("Connecting to MongoDB...")

	clientOptions := options.Client().ApplyURI("mongodb://root:example@database:27017")

	client, err := mongo.Connect(context.TODO(), clientOptions)

	if err != nil {
		log.Fatalf("Error initializing MongoDB connection: %v", err)
	}

	err = client.Ping(context.TODO(), nil)

	if err != nil {
		log.Fatalf("Error pinging established MongoDB connection: %v", err)
	}

	fmt.Println("Connected to MongoDB!")

	datasource := Datasource{
		*client,
		database,
	}

	datasource.loadLocalData()

	return datasource
}

func (c *Datasource) loadLocalData() {
	localData := GetData()
	actorsCollection := c.GetActorsCollection()

	for _, actor := range localData.Actors {
		res := actorsCollection.FindOne(context.TODO(), bson.D{{Key: "_id", Value: actor.ID}})
		if err := res.Err(); err != nil {
			if err == mongo.ErrNoDocuments {
				fmt.Printf("Loading actor: %v\n", actor)
				_, err := actorsCollection.InsertOne(context.TODO(), actor)

				if err != nil {
					log.Fatalf("Failed to load data for actor: %v, err: %v", actor, err)
				}
			} else {
				log.Fatalf("Failed to retrieve data about actor: %v, err: %v", actor, err)
			}
		} else {
			fmt.Printf("Skipping already loaded actor: %v\n", actor.ID)
		}
	}

	moviesCollection := c.GetMoviesCollection()
	for _, movie := range localData.Movies {
		res := moviesCollection.FindOne(context.TODO(), bson.D{{Key: "_id", Value: movie.ID}})
		if err := res.Err(); err != nil {
			if err == mongo.ErrNoDocuments {
				fmt.Printf("Loading movie: %v\n", movie)
				_, err := moviesCollection.InsertOne(context.TODO(), movie)

				if err != nil {
					log.Fatalf("Failed to load data for movie: %v, err: %v", movie, err)
				}
			} else {
				log.Fatalf("Failed to retrieve data about movie: %v, err: %v", movie, err)
			}
		} else {
			fmt.Printf("Skipping already loaded movie: %v\n", movie.ID)
		}
	}
}

func (c *Datasource) GetActorsCollection() *mongo.Collection {
	return c.client.Database(c.database).Collection("actors")
}

func (c *Datasource) GetMoviesCollection() *mongo.Collection {
	return c.client.Database(c.database).Collection("movies")
}
