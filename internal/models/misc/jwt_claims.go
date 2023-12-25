package misc

import "github.com/dgrijalva/jwt-go"

type Claims struct {
	UserId string `json:"user_id"`
	jwt.StandardClaims
}
