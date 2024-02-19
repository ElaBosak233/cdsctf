package captcha

import (
	"bytes"
	"github.com/elabosak233/cloudsdale/internal/config"
	"io"
	"k8s.io/apimachinery/pkg/util/json"
	"net/http"
)

type CloudflareTurnstile struct {
	URL       string
	SiteKey   string
	SecretKey string
}

func NewCloudflareTurnstile() ICaptcha {
	return &CloudflareTurnstile{
		URL:       config.AppCfg().Captcha.Turnstile.URL,
		SiteKey:   config.AppCfg().Captcha.Turnstile.SiteKey,
		SecretKey: config.AppCfg().Captcha.Turnstile.SecretKey,
	}
}

func (c *CloudflareTurnstile) Verify(token string, clientIP string) (success bool, err error) {
	type TurnstileRequest struct {
		SecretKey string `json:"secret"`
		Response  string `json:"response"`
		RemoteIP  string `json:"remoteip"`
	}
	requestBody, err := json.Marshal(
		TurnstileRequest{
			SecretKey: c.SecretKey,
			Response:  token,
			RemoteIP:  clientIP,
		},
	)
	result, err := http.Post(c.URL, "application/json", bytes.NewBuffer(requestBody))
	defer func() {
		_ = result.Body.Close()
	}()
	body, err := io.ReadAll(result.Body)
	var response map[string]interface{}
	err = json.Unmarshal(body, &response)
	success, ok := response["success"].(bool)
	if ok && success {
		return true, err
	}
	return false, err
}
