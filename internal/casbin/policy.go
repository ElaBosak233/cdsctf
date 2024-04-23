package casbin

import "go.uber.org/zap"

func initDefaultPolicy() {
	_, err := Enforcer.AddPolicies([][]string{
		{"admin", "/api/*", "GET"},
		{"admin", "/api/*", "POST"},
		{"admin", "/api/*", "PUT"},
		{"admin", "/api/*", "DELETE"},

		{"user", "/api/", "GET"},
		{"user", "/api/users/logout", "POST"},
		{"user", "/api/users/{id}", "PUT"},
		{"user", "/api/users/{id}", "DELETE"},
		{"user", "/api/teams/", "GET"},
		{"user", "/api/teams/", "POST"},
		{"user", "/api/teams/{id}", "PUT"},
		{"user", "/api/teams/{id}", "GET"},
		{"user", "/api/teams/{id}", "DELETE"},
		{"user", "/api/teams/{id}/invite", "PUT"},
		{"user", "/api/teams/{id}/invite", "GET"},
		{"user", "/api/teams/{id}/users/{user_id}", "DELETE"},
		{"user", "/api/teams/{id}/join", "POST"},
		{"user", "/api/teams/{id}/leave", "DELETE"},
		{"user", "/api/challenges/*", "GET"},
		{"user", "/api/games/", "GET"},
		{"user", "/api/games/{id}", "GET"},
		{"user", "/api/games/{id}/scoreboard", "GET"},
		{"user", "/api/games/{id}/challenges", "GET"},
		{"user", "/api/games/{id}/teams", "GET"},
		{"user", "/api/games/{id}/teams", "POST"},
		{"user", "/api/games/{id}/teams/{team_id}", "GET"},
		{"user", "/api/games/{id}/notices", "GET"},
		{"user", "/api/submissions/", "GET"},
		{"user", "/api/submissions/", "POST"},
		{"user", "/api/pods/", "GET"},
		{"user", "/api/pods/", "POST"},
		{"user", "/api/pods/{id}", "GET"},
		{"user", "/api/pods/{id}", "PUT"},
		{"user", "/api/pods/{id}", "DELETE"},

		{"guest", "/api/", "GET"},
		{"guest", "/api/configs/", "GET"},
		{"guest", "/api/categories/", "GET"},
		{"guest", "/api/users/", "GET"},
		{"guest", "/api/users/register", "POST"},
		{"guest", "/api/users/login", "POST"},
		{"guest", "/api/games/{id}/broadcast", "GET"},
		{"guest", "/api/proxies/{id}", "GET"},
		{"guest", "/api/media/*", "GET"},
		{"guest", "/api/groups/", "GET"},
	})

	_, err = Enforcer.AddGroupingPolicies([][]string{
		{"user", "guest"},
		{"admin", "user"},
	})

	if err != nil {
		zap.L().Fatal("Casbin init default policy error.", zap.Error(err))
	}
}
