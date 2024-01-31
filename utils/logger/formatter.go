package logger

import (
	"bytes"
	"fmt"
	"github.com/duke-git/lancet/v2/strutil"
	"github.com/sirupsen/logrus"
)

type IFormatter struct{}

func (i *IFormatter) Format(entry *logrus.Entry) ([]byte, error) {
	var b *bytes.Buffer
	if entry.Buffer != nil {
		b = entry.Buffer
	} else {
		b = &bytes.Buffer{}
	}
	if entry.Message != "" {
		build := fmt.Sprintf(
			"[%s] [%s] %s\n",
			strutil.UpperKebabCase(entry.Level.String()),
			entry.Time.Format("2006-01-02 15:04:05"),
			entry.Message)
		b.WriteString(build)
	}
	return b.Bytes(), nil
}
