# Mavinote Web
Web application of Mavinote. Web application does not have a **Local** account like Android and iOS have. It is designed to work on **Mavinote** accounts.

## Prerequisites
Before starting the development, you need to do:

* Complete **wasm prerequisites** described in [reax](https://github.com/bwqr/mavinote/tree/main/reax) project.
* Install project dependencies with `npm install` (or `pnpm install` or `yarn`).

## Configuration
Project has two files that define the build time configurations, `.env.development` for development and `.env.production` for production.
Right now, project has these variables defined in the configuration files:

* **VITE_API_URL**: This variable contains the URL of backend service. An example is `http://127.0.0.1:8050/api`.

## Development
You can start the development server with
```sh
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Deployment
You can get production files manually or create a docker image which contains the project as ready to serve.

* To get production files manually, you can run with
```sh
npm run build
```
The build files are located in `build` directory.

* To create a docker image, your current working directory must be the root of the repository since docker needs to access **reax** project.
Then you can create docker image with
```sh
cd <root-of-repository>
docker build -f svelte/Dockerfile -t <docker-image-tag> .
```
