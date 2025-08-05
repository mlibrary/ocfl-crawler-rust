FROM rust:1.88-bookworm AS base

ARG UID=1000
ARG GID=1000

RUN groupadd -g ${GID} -o app
RUN useradd -m -d /home/app -u ${UID} -g ${GID} -o -s /bin/bash app

RUN apt-get update -yqq && apt-get install -yqq --no-install-recommends \
  vim-tiny

WORKDIR /usr/src/app

CMD ["tail", "-f", "/dev/null"]

FROM base AS development

RUN apt-get update -yqq && apt-get install -yqq --no-install-recommends \
  git

# Switch to the non-root user "user"
USER app


