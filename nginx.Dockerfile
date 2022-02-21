FROM nginx:latest

COPY nginx.conf /etc/nginx/nginx.conf
COPY /etc/letsencrypt/live/websocket.noburu.app/fullchain.pem /etc/letsencrypt/live/websocket.noburu.app/fullchain.pem
COPY /etc/letsencrypt/live/websocket.noburu.app/fullchain.pem /etc/letsencrypt/live/websocket.noburu.app/fullchain.pem
COPY /etc/letsencrypt/options-ssl-nginx.conf /etc/letsencrypt/options-ssl-nginx.conf
COPY /etc/letsencrypt/ssl-dhparams.pem /etc/letsencrypt/ssl-dhparams.pem
