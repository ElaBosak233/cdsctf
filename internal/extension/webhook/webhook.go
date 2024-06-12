package webhook

import (
	"github.com/elabosak233/cloudsdale/internal/model"
)

func POST(webhooks []*model.Webhook, object interface{}) {
	for _, webhook := range webhooks {
		switch webhook.Type {
		case "application/x-www-form-urlencoded":
			break
		default:
		case "application/json":
			break
		}
	}
}
