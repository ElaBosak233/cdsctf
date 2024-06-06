package controller

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/extension/proxy"
	"github.com/gin-gonic/gin"
	"github.com/gorilla/websocket"
	"go.uber.org/zap"
	"net/http"
	"time"
)

type IProxyController interface {
	Connect(ctx *gin.Context)
}

type ProxyController struct {
}

func NewProxyController() IProxyController {
	return &ProxyController{}
}

// Connect
// TCP over WebSocket in Cloudsdale requires a complete Websocket header to establish a connection.
func (p *ProxyController) Connect(ctx *gin.Context) {
	id := ctx.Param("id")
	upgrade := websocket.Upgrader{
		CheckOrigin: func(r *http.Request) bool {
			return true
		},
	}

	conn, err := upgrade.Upgrade(ctx.Writer, ctx.Request, nil)
	if err != nil {
		zap.L().Error("Failed to upgrade to WebSocket", zap.Error(err))
		return
	}
	defer func(conn *websocket.Conn) {
		_ = conn.Close()
	}(conn)
	_ = conn.SetReadDeadline(time.Time{})
	_ = conn.SetWriteDeadline(time.Time{})
	if ws, ok := proxy.WSProxyMap[id]; ok {
		zap.L().Info(fmt.Sprintf("Websocket proxy found %s -> %s", id, ws.Target))
		ws.Handle(conn)
	}
}
