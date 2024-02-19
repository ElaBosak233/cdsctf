package proxy

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/google/gopacket"
	"github.com/google/gopacket/layers"
	"github.com/google/gopacket/pcapgo"
	"github.com/google/uuid"
	"github.com/gorilla/websocket"
	"go.uber.org/zap"
	"net"
	"net/http"
	"os"
	"path"
	"strings"
	"time"
)

var (
	WSProxyMap = make(map[string]*WSProxy)
)

type WSProxy struct {
	Listen   string
	Target   string
	Upgrader websocket.Upgrader
}

func NewWSProxy(target string) IProxy {
	return &WSProxy{
		Target: target,
		Upgrader: websocket.Upgrader{
			CheckOrigin: func(r *http.Request) bool {
				return true
			},
		},
	}
}

func (w *WSProxy) Setup() {
	w.Listen = uuid.NewString()
	WSProxyMap[w.Listen] = w
}

func (w *WSProxy) Handle(conn *websocket.Conn) {
	switch config.AppCfg().Container.TrafficCapture.Enabled {
	case true:
		w.handleInTrafficCapture(conn)
	case false:
		w.handle(conn)
	}
}

func (w *WSProxy) Close() {
	delete(WSProxyMap, w.Listen)
}

func (w *WSProxy) handle(conn *websocket.Conn) {
	// 创建一个TCP连接到目标地址
	tcpConn, err := net.Dial("tcp", w.Target)
	if err != nil {
		zap.L().Error("Failed to connect to target.", zap.Error(err))
		return
	}
	defer func(tcpConn net.Conn) {
		_ = tcpConn.Close()
	}(tcpConn)

	// websocket -> tcp
	go func() {
		for {
			messageType, message, err := conn.ReadMessage()
			if err != nil {
				zap.L().Debug("WebSocket read error.", zap.Error(err))
				return
			}
			if _, err := tcpConn.Write(message); err != nil {
				zap.L().Debug("TCP connection write error.", zap.Error(err))
				return
			}
			if messageType == websocket.CloseMessage {
				zap.L().Debug("WebSocket closed by client.")
				return
			}
		}
	}()

	// tcp -> websocket
	for {
		buf := make([]byte, 1024)
		n, err := tcpConn.Read(buf)
		if err != nil {
			zap.L().Debug("TCP connection read error.", zap.Error(err))
			return
		}
		if err := conn.WriteMessage(websocket.TextMessage, buf[:n]); err != nil {
			zap.L().Debug("WebSocket write error.", zap.Error(err))
			return
		}
	}
}

func (w *WSProxy) handleInTrafficCapture(conn *websocket.Conn) {
	tcpConn, err := net.Dial("tcp", w.Target)
	if err != nil {
		zap.L().Error("Failed to connect to target.", zap.Error(err))
		return
	}
	defer func(tcpConn net.Conn) {
		_ = tcpConn.Close()
	}(tcpConn)

	clientIP := strings.Split(conn.RemoteAddr().String(), ":")[0]
	targetIP := strings.Split(w.Target, ":")[0]
	targetPort := strings.Split(w.Target, ":")[1]
	f, err := os.Create(
		path.Join(
			config.AppCfg().Container.TrafficCapture.Path,
			fmt.Sprintf(
				"%s-%s-%s-%s.pcap",
				clientIP,
				targetIP,
				targetPort,
				time.Now().Format("2006-01-02-15-04-05"),
			),
		),
	)
	if err != nil {
		zap.L().Error("Failed to create pcap file.", zap.Error(err))
		return
	}
	defer func(f *os.File) {
		_ = f.Close()
	}(f)

	writer := pcapgo.NewWriter(f)
	if _err := writer.WriteFileHeader(1024, layers.LinkTypeEthernet); _err != nil {
		zap.L().Error("Failed to write PCAP file header.", zap.Error(_err))
		return
	}

	go func() {
		for {
			messageType, message, _err := conn.ReadMessage()
			if _err != nil {
				zap.L().Debug("WebSocket read error.", zap.Error(_err))
				return
			}
			_, _err = tcpConn.Write(message)
			if _err != nil {
				zap.L().Debug("TCP connection write error.", zap.Error(_err))
				return
			}
			if messageType == websocket.CloseMessage {
				zap.L().Debug("WebSocket closed by client.")
				return
			}
			if __err := writer.WritePacket(gopacket.CaptureInfo{
				CaptureLength: len(message),
				Length:        len(message),
				Timestamp:     time.Now(),
			}, message); __err != nil {
				zap.L().Debug("Failed to write packet to PCAP file.", zap.Error(__err))
				return
			}
		}
	}()

	buf := make([]byte, 1024)
	for {
		n, _err := tcpConn.Read(buf)
		if _err != nil {
			zap.L().Debug("TCP connection read error.", zap.Error(_err))
			return
		}
		_err = conn.WriteMessage(websocket.TextMessage, buf[:n])
		if _err != nil {
			zap.L().Debug("WebSocket write error.", zap.Error(_err))
			return
		}
		if __err := writer.WritePacket(gopacket.CaptureInfo{
			CaptureLength: n,
			Length:        n,
			Timestamp:     time.Now(),
		}, buf[:n]); __err != nil {
			zap.L().Debug("Failed to write packet to PCAP file.", zap.Error(__err))
			return
		}
	}
}

func (w *WSProxy) GetEntry() string {
	return w.Listen
}
