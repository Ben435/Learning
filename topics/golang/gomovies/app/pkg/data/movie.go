package data

import (
	"errors"
	"fmt"
)

type MovieData struct {
	ID       string   `json:"id"`
	Title    string   `json:"title"`
	ActorIDs []string `json:"actors"`
}

type Movie struct {
	ID     string
	Title  string
	Actors []Actor
}

func GetMovie(id string) (Movie, error) {
	data := GetData()

	movies := data.Movies
	var movieData *MovieData

	for i := 0; i < len(movies); i++ {
		if movies[i].ID == id {
			movieData = &movies[i]
		}
	}

	if movieData == nil {
		return Movie{}, errors.New("Failed to find movie")
	}

	actorIDs := movieData.ActorIDs
	actors := make([]Actor, len(movieData.ActorIDs))
	for i := 0; i < len(actorIDs); i++ {
		actor, err := GetActor(actorIDs[i])
		if err != nil {
			return Movie{}, errors.New(fmt.Sprintf("Failed to find actor id %v in movie %v", actorIDs[i], movieData.ID))
		}

		actors[i] = actor
	}

	return Movie{
		ID:     movieData.ID,
		Title:  movieData.Title,
		Actors: actors,
	}, nil
}
