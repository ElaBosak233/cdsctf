package utils

import (
	"bytes"
	"crypto/ed25519"
	"crypto/sha256"
	"encoding/base64"
	"encoding/gob"
	"fmt"
	"github.com/google/uuid"
	"strings"
)

func Sign(privateKey string, message string) (signature string, err error) {
	pri, err := base64.StdEncoding.DecodeString(privateKey)
	return base64.StdEncoding.EncodeToString(
		ed25519.Sign(pri, []byte(message)),
	), err
}

func HyphenlessUUID() string {
	return strings.Replace(uuid.NewString(), "-", "", -1)
}

func HashStruct(s interface{}) string {
	var buf bytes.Buffer
	enc := gob.NewEncoder(&buf)
	err := enc.Encode(s)
	if err != nil {
		return "null"
	}
	hash := sha256.Sum256(buf.Bytes())
	return fmt.Sprintf("%x", hash)
}
