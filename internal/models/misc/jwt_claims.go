package misc

import "github.com/dgrijalva/jwt-go"

type Claims struct {
	UserId string `json:"user_id"`
	Role   int    `json:"role"`
	jwt.StandardClaims
}
