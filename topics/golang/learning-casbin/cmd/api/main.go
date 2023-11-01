package main

import (
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"time"

	"github.com/alexedwards/scs/v2"
	"github.com/casbin/casbin/v2"

	"learning_casbin/model"
	"learning_casbin/route"
)

func main() {
	authEnforcer, err := casbin.NewEnforcer("./auth_model.conf", "./policy.csv")
	if err != nil {
		slog.Error("failed to init casbin enforcer: %w", err)
		os.Exit(-1)
	}

	sessionManager := scs.New()
	sessionManager.Lifetime = 30 * time.Minute

	users := model.NewUsers()
	mux := http.NewServeMux()

	handler := route.NewHandler(sessionManager, users)

	mux.HandleFunc("/login", handler.LoginHandler())
	mux.HandleFunc("/logout", handler.LogoutHandler())
	mux.HandleFunc("/member/current", handler.CurrentMemberHandler())
	mux.HandleFunc("/member/name", handler.MemberNameHandler())
	mux.HandleFunc("/admin/stuff", handler.AdminHandler())

	port, exists := os.LookupEnv("PORT")
	if !exists {
		port = "8080"
	}
	slog.Info("Server started on localhost", "port", port)
	err = http.ListenAndServe(fmt.Sprintf(":%s", port), sessionManager.LoadAndSave(route.Authorizer(authEnforcer, sessionManager, users)(mux)))

	if err != nil {
		slog.Error("listen and serve crashed", "error", err)
	}
}
