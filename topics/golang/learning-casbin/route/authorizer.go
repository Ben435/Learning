package route

import (
	"errors"
	"fmt"
	"log/slog"
	"net/http"

	"github.com/alexedwards/scs/v2"
	"github.com/casbin/casbin/v2"

	"learning_casbin/model"
)

func Authorizer(e *casbin.Enforcer, sessionManager *scs.SessionManager, users model.Users) func(next http.Handler) http.Handler {
	return func(next http.Handler) http.Handler {
		fn := func(w http.ResponseWriter, r *http.Request) {
			role := sessionManager.GetString(r.Context(), "role")

			if role == "" {
				role = "anonymous"
			}

			slog.Info("found user", "role", role, "in-session", sessionManager.Exists(r.Context(), "role"))

			// if it's a member, check if the user still exists
			if role == "member" {
				uid := sessionManager.GetInt(r.Context(), "userID")
				if uid == 0 {
					writeError(http.StatusInternalServerError, "ERROR", w, fmt.Errorf("failed to find userID in session"))
					return
				}
				exists := users.Exists(uid)
				if !exists {
					writeError(http.StatusForbidden, "FORBIDDEN", w, errors.New("user does not exist"))
					return
				}
			}

			// casbin rule enforcing
			res, err := e.Enforce(role, r.URL.Path, r.Method)
			if err != nil {
				writeError(http.StatusInternalServerError, "ERROR", w, err)
				return
			}
			if res {
				next.ServeHTTP(w, r)
			} else {
				writeError(http.StatusForbidden, "FORBIDDEN", w, errors.New("unauthorized"))
				return
			}
		}

		return http.HandlerFunc(fn)
	}
}
