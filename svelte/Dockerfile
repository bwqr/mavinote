# This build file should be used from the repo's root directory since it builds the reax
FROM rust:1-slim-bullseye as wasm-builder

WORKDIR /usr/local/src/mavinote/reax

COPY reax .

WORKDIR /usr/local/src/mavinote/reax

RUN apt-get update && apt-get install -y curl && curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && wasm-pack build --target web wasm

FROM alpine as svelte-builder

WORKDIR /usr/local/src/mavinote/svelte

COPY svelte .
COPY --from=wasm-builder /usr/local/src/mavinote/reax/wasm/pkg wasm

RUN apk update && apk add nodejs npm && npm i && npm run build && cp package.json build/package.json

FROM alpine

WORKDIR /usr/local/app/mavinote/svelte

COPY --from=svelte-builder /usr/local/src/mavinote/svelte/build .

RUN apk update && apk add nodejs npm && npm i --omit=dev

EXPOSE 3000

CMD ["node", "/usr/local/app/mavinote/svelte/index.js"]
