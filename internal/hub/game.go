package hub

import (
	"github.com/gorilla/websocket"
	"go.uber.org/zap"
	"net/http"
	"sync"
)

type GameHub struct {
	Upgrader  websocket.Upgrader
	Clients   map[*websocket.Conn]bool
	BroadCast chan interface{}
	M         sync.RWMutex
}

var gameHubs = make(map[int64]*GameHub)
var gameHubsLock sync.Mutex

func ServeGameHub(w http.ResponseWriter, r *http.Request, gameID int64) {
	gameHubsLock.Lock()
	gameHub, ok := gameHubs[gameID]
	if !ok {
		g := &GameHub{
			Clients:   make(map[*websocket.Conn]bool),
			BroadCast: make(chan interface{}),
			M:         sync.RWMutex{},
		}
		gameHubs[gameID] = g
		gameHub = g
		go handleGameHub(g)
	}
	gameHubsLock.Unlock()
	conn, _ := upgrader.Upgrade(w, r, nil)

	defer func() {
		gameHub.M.Lock()
		if conn != nil {
			if _, exists := gameHub.Clients[conn]; exists {
				delete(gameHub.Clients, conn)
			}
		}
		gameHub.M.Unlock()
		_ = conn.Close()
	}()
	gameHub.M.Lock()
	gameHub.Clients[conn] = true
	gameHub.M.Unlock()
	for {
		_, _, err := conn.ReadMessage()
		if err != nil {
			break
		}
	}
}

func handleGameHub(gameHub *GameHub) {
	for {
		msg := <-gameHub.BroadCast
		gameHub.M.RLock()
		for client := range gameHub.Clients {
			err := client.WriteJSON(msg)
			if err != nil {
				zap.L().Error("", zap.Error(err))
				_ = client.Close()
				delete(gameHub.Clients, client)
				gameHub.M.RUnlock()
				break
			}
		}
		gameHub.M.RUnlock()
	}
}

func SendMsg(gameID int64, msg interface{}) {
	gameHubsLock.Lock()
	gameHub, ok := gameHubs[gameID]
	gameHubsLock.Unlock()
	if ok {
		gameHub.BroadCast <- msg
	}
}
