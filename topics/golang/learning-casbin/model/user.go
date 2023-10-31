package model

type Role string

const (
	Anonymous Role = "anonymous"
	Member    Role = "member"
	Admin     Role = "admin"
)

type User struct {
	ID   int
	Name string
	Role Role
}

type Users []User

func (u Users) Exists(id int) bool {
	for _, user := range u {
		if user.ID == id {
			return true
		}
	}
	return false
}

func (u Users) FindByName(name string) *User {
	for _, user := range u {
		if user.Name == name {
			return &user
		}
	}
	return nil
}

func NewUsers() Users {
	return Users{
		{
			ID:   1,
			Name: "Gandalf",
			Role: Admin,
		},
		{
			ID:   2,
			Name: "Frodo",
			Role: Member,
		},
		{
			ID:   3,
			Name: "Smeagle",
			Role: Anonymous,
		},
	}
}
