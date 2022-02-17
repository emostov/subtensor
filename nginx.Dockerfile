FROM nginx:latest

COPY nginx.conf /etc/nginx/nginx.conf
COPY subtensor.crt /etc/ssl/certs/subtensor.crt
COPY subtensor.key /etc/ssl/private/subtensor.key
