FROM golang:latest AS backend

COPY ./ /app

WORKDIR /app

RUN make build

FROM node:20 AS frontend

COPY ./client /app

WORKDIR /app

RUN npm install
RUN npm run build

FROM alpine:3.14

COPY --from=backend /app/build/cloudsdale /app/cloudsdale
COPY --from=frontend /app/dist /app/dist

WORKDIR /app

VOLUME /var/run/docker.sock

EXPOSE 8888

CMD ["./cloudsdale"]