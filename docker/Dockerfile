FROM alpine:3.9
EXPOSE 8000
ENV REVEAL_VERSION=3.6.0 \
    HIGHLIGHT_JS_VERSION=9.11.0
RUN : \
 && apk add --no-cache --virtual .build-deps \
    curl ca-certificates \
    g++ \
    make \
    python \
    tar \
 && apk add --no-cache nodejs npm \
 \
 && mkdir -p /data \
 && curl -ssL "https://github.com/hakimel/reveal.js/archive/${REVEAL_VERSION}.tar.gz" \
    | tar -xzf - -C /data --strip-components 1 \
 && (cd data && npm install) \
 \
 && curl -ssL "https://github.com/highlightjs/highlight.js/archive/${HIGHLIGHT_JS_VERSION}.tar.gz" \
    | tar -xzf - -C /data/lib/css --strip-components 3 --wildcards 'highlight.js-*/src/styles' \
 \
 && apk del --no-cache .build-deps \
 && npm cache clean --force 2>/dev/null \
 && rm -rf /root/.node-gyp /tmp/npm* /tmp/phantomjs \
 && :
WORKDIR /data
CMD ["npm", "run", "start"]
COPY Dockerfile build run /src/
