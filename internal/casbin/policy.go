package casbin

func initDefaultPolicy() {
	_, _ = Enforcer.AddPolicies([][]string{
		{"admin", "/api/*", "GET"},
	})
}
