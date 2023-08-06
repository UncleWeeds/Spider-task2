FROM nginx

RUN apt-get update && apt-get install vim -y

COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

ENTRYPOINT ["nginx", "-g", "daemon off;"]


