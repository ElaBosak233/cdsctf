FROM golang:1.22-alpine as builder

RUN apk add --no-cache git python3 py3-pip gcc musl-dev nodejs npm

COPY ./ /app/backend
COPY ./client /app/frontend

WORKDIR /app/backend

RUN python3 scripts/buildtool.py build
RUN cp build/cloudsdale /go/bin/cloudsdale

WORKDIR /app/frontend

RUN npm install
RUN npm run build
RUN cp -r dist /go/dist

FROM alpine:3.14

COPY --from=builder /go/bin/cloudsdale /app/cloudsdale
COPY --from=builder /go/dist /app/dist

WORKDIR /app

VOLUME /var/run/docker.sock

EXPOSE 8888

CMD ["./cloudsdale"]