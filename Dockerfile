FROM nginx:alpine-slim

COPY index.html /usr/share/nginx/html/index.html
COPY nginx.conf /etc/nginx/nginx.conf
COPY ./dist/. /usr/share/nginx/html/