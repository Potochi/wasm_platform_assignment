FROM node:18.1-buster-slim as builder

RUN npm install -g pnpm

WORKDIR /build

COPY ./dashboard /build/dashboard

WORKDIR /build/dashboard

RUN pnpm install
RUN pnpm run build
RUN mv ./dist /dist


FROM nginx:1.24.0-alpine-slim

WORKDIR /
COPY --from=builder /dist /usr/share/nginx/html

EXPOSE 80
