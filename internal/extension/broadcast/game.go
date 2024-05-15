package broadcast

import (
	"github.com/gorilla/websocket"
	"go.uber.org/zap"
	"net/http"
	"sync"
)

type GameBroadcast struct {
	Upgrader  websocket.Upgrader
	Clients   map[*websocket.Conn]bool
	BroadCast chan interface{}
	M         sync.RWMutex
}

var gameBroadcasts = make(map[uint]*GameBroadcast)
var gameBroadcastLock sync.Mutex

func ServeGameHub(w http.ResponseWriter, r *http.Request, gameID uint) {
	gameBroadcastLock.Lock()
	gameBroadcast, ok := gameBroadcasts[gameID]
	if !ok {
		g := &GameBroadcast{
			Clients:   make(map[*websocket.Conn]bool),
			BroadCast: make(chan interface{}),
			M:         sync.RWMutex{},
		}
		gameBroadcasts[gameID] = g
		gameBroadcast = g
		go handleGameBroadcast(g)
	}
	gameBroadcastLock.Unlock()
	conn, _ := Upgrader.Upgrade(w, r, nil)

	defer func() {
		gameBroadcast.M.Lock()
		if conn != nil {
			if _, exists := gameBroadcast.Clients[conn]; exists {
				delete(gameBroadcast.Clients, conn)
			}
		}
		gameBroadcast.M.Unlock()
		_ = conn.Close()
	}()
	gameBroadcast.M.Lock()
	gameBroadcast.Clients[conn] = true
	gameBroadcast.M.Unlock()
	for {
		_, _, err := conn.ReadMessage()
		if err != nil {
			break
		}
	}
}

func handleGameBroadcast(gameBroadcast *GameBroadcast) {
	for {
		msg := <-gameBroadcast.BroadCast
		gameBroadcast.M.RLock()
		for client := range gameBroadcast.Clients {
			err := client.WriteJSON(msg)
			if err != nil {
				zap.L().Error("", zap.Error(err))
				_ = client.Close()
				delete(gameBroadcast.Clients, client)
				gameBroadcast.M.RUnlock()
				break
			}
		}
		gameBroadcast.M.RUnlock()
	}
}

func SendGameMsg(gameID uint, msg interface{}) {
	gameBroadcastLock.Lock()
	gameHub, ok := gameBroadcasts[gameID]
	gameBroadcastLock.Unlock()
	if ok {
		gameHub.BroadCast <- msg
	}
}
