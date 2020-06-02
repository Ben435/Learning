package data

import (
	"encoding/json"
	"io/ioutil"
	"log"
)

type LoadedData struct {
	loaded bool
	data   Data
}

type Data struct {
	Actors []Actor     `json:"actors"`
	Movies []MovieData `json:"movies"`
}

var loadedData = LoadedData{loaded: false}

func GetData() Data {
	if !loadedData.loaded {
		content, err := ioutil.ReadFile("dummy_data.json")
		if err != nil {
			log.Fatalf("Failed to load data: %v", err)
		}

		data := Data{}
		err = json.Unmarshal(content, &data)

		if err != nil {
			log.Fatalf("Failed to unmarshall data: %v", err)
		}

		log.Printf("Loaded data: %v", data)

		loadedData.data = data
		loadedData.loaded = true
	}

	return loadedData.data
}
