package proxy

import (
	"errors"
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/google/gopacket"
	"github.com/google/gopacket/layers"
	"github.com/google/gopacket/pcapgo"
	"github.com/phayes/freeport"
	"go.uber.org/zap"
	"io"
	"net"
	"os"
	"path"
	"strings"
	"time"
)

type TCPProxy struct {
	Listen   string
	Target   string
	listener net.Listener
}

func NewTCPProxy(target string) IProxy {
	return &TCPProxy{
		Target: target,
	}
}

func (t *TCPProxy) Setup() {
	port, err := freeport.GetFreePort()
	if err != nil {
		zap.L().Error("Failed to get free port for proxy.", zap.Error(err))
	}
	t.Listen = fmt.Sprintf("%s:%d", config.AppCfg().Container.Proxy.TCP.Entry, port)
	t.listener, err = net.Listen("tcp", t.Listen)
	if err != nil {
		zap.L().Error(fmt.Sprintf("Failed to listen on %s: %v", t.Listen, err))
	}

	zap.L().Info(fmt.Sprintf("Proxy listening on %s, forwarding to %s", t.Listen, t.Target))
	go func() {
		for {
			conn, err := t.listener.Accept()
			if err != nil {
				if errors.Is(err, net.ErrClosed) {
					return // listener 已经关闭
				}
				zap.L().Error(fmt.Sprintf("Failed to accept connection: %v", err))
				continue
			}
			switch config.AppCfg().Container.TrafficCapture.Enabled {
			case true:
				go t.HandleInTrafficCapture(conn, t.Target)
			case false:
				go t.Handle(conn, t.Target)
			}
		}
	}()
}

func (t *TCPProxy) Close() {
	_ = t.listener.Close()
}

func (t *TCPProxy) Handle(clientConn net.Conn, target string) {
	targetConn, err := net.Dial("tcp", target)
	if err != nil {
		zap.L().Error(fmt.Sprintf("Failed to connect to target %s", target), zap.Error(err))
		_ = clientConn.Close()
		return
	}
	defer func(targetConn net.Conn) {
		_ = targetConn.Close()
	}(targetConn)

	go func() {
		// Client -> Target
		_, _ = io.Copy(targetConn, clientConn)
		_ = targetConn.Close()
	}()
	// Target -> Client
	_, _ = io.Copy(clientConn, targetConn)
	_ = clientConn.Close()
}

func (t *TCPProxy) HandleInTrafficCapture(clientConn net.Conn, target string) {
	targetConn, err := net.Dial("tcp", target)
	if err != nil {
		zap.L().Error(fmt.Sprintf("Failed to connect to target %s", target), zap.Error(err))
		_ = clientConn.Close()
		return
	}
	defer func(targetConn net.Conn) {
		_ = targetConn.Close()
	}(targetConn)

	clientAddr := strings.Split(clientConn.RemoteAddr().String(), ":")[0]
	targetAddr := strings.Split(targetConn.RemoteAddr().String(), ":")[0]
	targetPort := strings.Split(targetConn.RemoteAddr().String(), ":")[1]
	zap.L().Info(fmt.Sprintf("Proxying from %s to %s:%s", clientAddr, targetAddr, targetPort))
	// Create a new pcap writer
	f, err := os.Create(
		path.Join(config.AppCfg().Container.TrafficCapture.Path,
			fmt.Sprintf(
				"%s-%s-%s-%s.pcap",
				clientAddr,
				targetAddr,
				targetPort,
				time.Now().Format("2006-01-02-15-04-05"),
			),
		),
	)
	if err != nil {
		zap.L().Error("Failed to create pcap file.", zap.Error(err))
	}
	w := pcapgo.NewWriter(f)
	_ = w.WriteFileHeader(1024, layers.LinkTypeEthernet)

	go func() {
		// Client -> Target
		buf := make([]byte, 1024)
		for {
			n, err := clientConn.Read(buf)
			if err != nil {
				break
			}
			_, _ = targetConn.Write(buf[:n])
			_ = w.WritePacket(gopacket.CaptureInfo{
				CaptureLength: n,
				Length:        n,
				Timestamp:     time.Now(),
			}, buf[:n])
			_ = f.Sync()
		}
		_ = targetConn.Close()
	}()
	// Target -> Client
	buf := make([]byte, 1024)
	for {
		n, err := targetConn.Read(buf)
		if err != nil {
			break
		}
		_, _ = clientConn.Write(buf[:n])
		_ = w.WritePacket(gopacket.CaptureInfo{
			CaptureLength: n,
			Length:        n,
			Timestamp:     time.Now(),
		}, buf[:n])
		_ = f.Sync()
	}
	_ = clientConn.Close()
	defer func(f *os.File) {
		_ = f.Close()
	}(f)
}

func (t *TCPProxy) GetEntry() string {
	return t.Listen
}
