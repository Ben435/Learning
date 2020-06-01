package gograph

import (
	"log"
	"os"
	"strings"
)

// DebugMode indicates if in debug mode (boolean)
const DebugMode = "DEBUG"

/*
GetConfigVar fetches config variable from environment (config, env vars, etc.)
	Should only be called with const variables defined in this file
*/
func GetConfigVar(envVar string) string {
	return os.Getenv(envVar)
}

/*
GetBoolConfigVar fetches config via GetConfigVar, and translates as a bool.
	Will default to false if could not parse the config var
*/
func GetBoolConfigVar(envVar string) bool {
	val := GetConfigVar(envVar)

	truthyVals := []string{"1", "true", "yes", "on"}
	falsyVals := []string{"0", "false", "no", "off"}

	for i := 0; i < len(truthyVals); i++ {
		if strings.EqualFold(val, truthyVals[i]) {
			return true
		}
	}

	for i := 0; i < len(truthyVals); i++ {
		if strings.EqualFold(val, falsyVals[i]) {
			return false
		}
	}

	log.Printf("Invalid boolean config var, defaulting to false: %v='%v'\n", envVar, val)
	return false
}
