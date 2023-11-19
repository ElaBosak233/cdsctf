package utils

import (
	"github.com/dgrijalva/jwt-go"
	"github.com/elabosak233/pgshub/model/misc"
	"time"
)

func GenerateJwtToken(id string) string {
	expirationTime := time.Now().Add(time.Duration(Cfg.Jwt.ExpirationTime) * time.Minute)
	jwtSecretKey := []byte(Cfg.Jwt.SecretKey)
	claims := &misc.Claims{
		Id: id,
		StandardClaims: jwt.StandardClaims{
			ExpiresAt: expirationTime.Unix(),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenString, err := token.SignedString(jwtSecretKey)
	ErrorPanic(err)
	return tokenString
}
