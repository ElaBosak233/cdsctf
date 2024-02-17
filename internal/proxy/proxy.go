package proxy

import (
	"fmt"
	"github.com/elabosak233/cloudsdale/internal/config"
	"github.com/google/gopacket"
	"github.com/google/gopacket/layers"
	"github.com/google/gopacket/pcapgo"
	"go.uber.org/zap"
	"io"
	"net"
	"os"
	"path"
	"strings"
	"time"
)

func Handle(clientConn net.Conn, target string) {
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

func HandleInTrafficCapture(clientConn net.Conn, target string) {
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
