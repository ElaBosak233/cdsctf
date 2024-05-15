package captcha

import (
	"github.com/elabosak233/cloudsdale/internal/extension/config"
)

type ICaptcha interface {
	Verify(token string, clientIP string) (success bool, err error)
}

func NewCaptcha() ICaptcha {
	switch config.AppCfg().Captcha.Provider {
	case "recaptcha":
		return NewGoogleRecaptcha()
	case "turnstile":
		return NewCloudflareTurnstile()
	}
	return nil
}
