package misc

import "github.com/dgrijalva/jwt-go"

type Claims struct {
	Id string `json:"id"`
	jwt.StandardClaims
}
