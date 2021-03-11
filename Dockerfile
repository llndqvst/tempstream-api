from debian:buster-slim

ARG APP=/usr/src/app

ENV APP_USER=tmpstream

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY target/release/tmp-stream ${APP}/tmp-stream

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

expose 8080
