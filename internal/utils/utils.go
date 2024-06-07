package utils

import (
	"bytes"
	"crypto/sha256"
	"encoding/gob"
	"fmt"
	"github.com/google/uuid"
	"strings"
)

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
