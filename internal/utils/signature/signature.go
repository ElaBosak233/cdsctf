package signature

import (
	"crypto/ed25519"
	"encoding/base64"
)

func Sign(privateKey string, message string) (signature string, err error) {
	pri, err := base64.StdEncoding.DecodeString(privateKey)
	return base64.StdEncoding.EncodeToString(
		ed25519.Sign(pri, []byte(message)),
	), err
}
