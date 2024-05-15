package captcha

import (
	"bytes"
	"encoding/json"
	"github.com/elabosak233/cloudsdale/internal/extension/config"
	"io"
	"net/http"
)

type GoogleRecaptcha struct {
	URL       string
	SiteKey   string
	SecretKey string
	Threshold float64
}

func NewGoogleRecaptcha() ICaptcha {
	return &GoogleRecaptcha{
		URL:       config.AppCfg().Captcha.ReCaptcha.URL,
		SiteKey:   config.AppCfg().Captcha.ReCaptcha.SiteKey,
		SecretKey: config.AppCfg().Captcha.ReCaptcha.SecretKey,
		Threshold: config.AppCfg().Captcha.ReCaptcha.Threshold,
	}
}

func (g *GoogleRecaptcha) Verify(token string, clientIP string) (success bool, err error) {
	type RecaptchaRequest struct {
		Secret   string `json:"secret"`
		Response string `json:"response"`
		RemoteIP string `json:"remoteip"`
	}
	requestBody, err := json.Marshal(
		RecaptchaRequest{
			Secret:   g.SecretKey,
			Response: token,
			RemoteIP: clientIP,
		},
	)
	result, err := http.Post(g.URL, "application/json", bytes.NewBuffer(requestBody))
	defer func() {
		_ = result.Body.Close()
	}()
	body, err := io.ReadAll(result.Body)
	var response map[string]interface{}
	err = json.Unmarshal(body, &response)
	success, ok := response["success"].(bool)
	score, ok := response["score"].(float64)
	if ok && success && score >= g.Threshold {
		return true, err
	}
	return false, err
}
