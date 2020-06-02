package data

import "errors"

type Actor struct {
	ID   string `json:"id"`
	Name string `json:"name"`
}

func GetActor(id string) (Actor, error) {

	data := GetData()
	actors := data.Actors

	for i := 0; i < len(actors); i++ {
		if actors[i].ID == id {
			return actors[i], nil
		}
	}

	return Actor{}, errors.New("Failed to find actor")
}
