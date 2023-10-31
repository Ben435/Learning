package route

import (
	"fmt"
	"net/http"

	"github.com/alexedwards/scs/v2"

	"learning_casbin/model"
)

type Handler struct {
	sessionManager *scs.SessionManager
	users          model.Users
}

func NewHandler(sessionManager *scs.SessionManager, users model.Users) *Handler {
	return &Handler{
		sessionManager: sessionManager,
		users:          users,
	}
}

func writeError(status int, message string, w http.ResponseWriter, err error) {
	w.WriteHeader(status)
	w.Write([]byte(fmt.Sprintf("%s: %s\n", message, err.Error())))
}

func writeSuccess(message string, w http.ResponseWriter) {
	w.WriteHeader(http.StatusOK)
	w.Write([]byte(fmt.Sprintf("%s\n", message)))
}

func (h *Handler) LoginHandler() http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		name := r.PostFormValue("name")
		user := h.users.FindByName(name)
		if user == nil {
			writeError(http.StatusUnauthorized, "failed to find user", w, fmt.Errorf("user not found: %s", name))
			return
		}

		// setup session
		if err := h.sessionManager.RenewToken(r.Context()); err != nil {
			writeError(http.StatusInternalServerError, "failed to renew token", w, err)
			return
		}
		h.sessionManager.Put(r.Context(), "userID", user.ID)
		h.sessionManager.Put(r.Context(), "role", string(user.Role))
		h.sessionManager.Commit(r.Context())
		writeSuccess("SUCCESS", w)
	})
}

func (h *Handler) LogoutHandler() http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if err := h.sessionManager.Destroy(r.Context()); err != nil {
			writeError(http.StatusInternalServerError, "failed to renew token", w, err)
			return
		}
		writeSuccess("SUCCESS", w)
	})
}

func (h *Handler) CurrentMemberHandler() http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		uid := h.sessionManager.GetInt(r.Context(), "userID")
		if uid == 0 {
			writeError(http.StatusInternalServerError, "ERROR", w, fmt.Errorf("failed to find userID in context"))
			return
		}
		writeSuccess(fmt.Sprintf("User with ID: %d", uid), w)
	})
}

func (h *Handler) MemberRoleHandler() http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		role := h.sessionManager.GetString(r.Context(), "role")
		if len(role) == 0 {
			writeError(http.StatusInternalServerError, "ERROR", w, fmt.Errorf("failed to find role in context"))
			return
		}
		writeSuccess(fmt.Sprintf("User with Role: %s", role), w)
	})
}

func (h *Handler) AdminHandler() http.HandlerFunc {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		writeSuccess("I'm an Admin!", w)
	})
}
