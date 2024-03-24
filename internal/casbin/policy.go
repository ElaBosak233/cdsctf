package casbin

import "go.uber.org/zap"

func initDefaultPolicy() {
	_, err := Enforcer.AddPolicies([][]string{
		{"admin", "/api/*", "GET"},
		{"admin", "/api/*", "POST"},
		{"admin", "/api/*", "PUT"},
		{"admin", "/api/*", "DELETE"},

		{"monitor", "/api/games", "POST"},
		{"monitor", "/api/games/{id}", "PUT"},
		{"monitor", "/api/games/{id}", "DELETE"},
		{"monitor", "/api/games/{id}/challenges", "POST"},
		{"monitor", "/api/games/{id}/teams/{team_id}", "PUT"},

		{"user", "/api/", "GET"},
		{"user", "/api/users/logout", "POST"},
		{"user", "/api/users/{id}", "PUT"},
		{"user", "/api/users/{id}", "DELETE"},
		{"user", "/api/challenges/", "GET"},
		{"user", "/api/categories/", "GET"},
		{"user", "/api/games/", "GET"},
		{"user", "/api/games/{id}", "GET"},
		{"user", "/api/games/{id}/scoreboard", "GET"},
		{"user", "/api/games/{id}/challenges", "GET"},
		{"user", "/api/games/{id}/teams", "GET"},
		{"user", "/api/games/{id}/teams", "POST"},
		{"user", "/api/games/{id}/teams/{team_id}", "GET"},
		{"user", "/api/submissions/", "POST"},
		{"user", "/api/pods/", "GET"},
		{"user", "/api/pods/", "POST"},
		{"user", "/api/pods/{id}", "GET"},
		{"user", "/api/pods/{id}", "PUT"},
		{"user", "/api/pods/{id}", "DELETE"},

		{"guest", "/api/", "GET"},
		{"guest", "/api/configs/", "GET"},
		{"guest", "/api/users/", "GET"},
		{"guest", "/api/users/token/{token}", "GET"},
		{"guest", "/api/users/register", "POST"},
		{"guest", "/api/users/login", "POST"},
		{"guest", "/api/games/{id}/broadcast", "GET"},
		{"guest", "/api/proxies/{id}", "GET"},
		{"guest", "/api/media/*", "GET"},
		{"guest", "/api/groups/", "GET"},
	})

	_, err = Enforcer.AddGroupingPolicies([][]string{
		{"user", "guest"},
		{"monitor", "user"},
		{"admin", "monitor"},
	})

	if err != nil {
		zap.L().Fatal("Casbin init default policy error.", zap.Error(err))
	}
}
