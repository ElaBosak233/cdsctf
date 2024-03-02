package config

import (
	"crypto/ed25519"
	"encoding/base64"
	"os"
)

var (
	sigCfg SignatureCfg
)

type SignatureCfg struct {
	PublicKey  string
	PrivateKey string
}

func SigCfg() *SignatureCfg {
	return &sigCfg
}

func InitSignatureCfg() {
	_, privateExists := os.Stat("signature.pem")
	_, publicExists := os.Stat("signature.pub")

	var privateKey []byte
	var publicKey []byte

	if privateExists != nil || publicExists != nil {
		publicKey, privateKey, _ = ed25519.GenerateKey(nil)
		_ = os.WriteFile("signature.pem", privateKey, 0600)
		_ = os.WriteFile("signature.pub", publicKey, 0600)
	} else {
		privateKey, _ = os.ReadFile("signature.pem")
		publicKey, _ = os.ReadFile("signature.pub")
	}

	sigCfg.PrivateKey = base64.StdEncoding.EncodeToString(privateKey)
	sigCfg.PublicKey = base64.StdEncoding.EncodeToString(publicKey)
}
