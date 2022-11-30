FROM node:16-alpine

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

RUN npm install -g ganache@7.5.0

CMD ["ganache", "--hostname", "0.0.0.0"]