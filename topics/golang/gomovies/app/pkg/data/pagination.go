package data

type PageInfo struct {
	HasNextPage     bool
	HasPreviousPage bool
	StartCursor     string
	EndCursor       string
}
