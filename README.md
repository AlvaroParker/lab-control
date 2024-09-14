# Production installation

You can use the installation script available for server deployment (it won't install the fingerprint component)

On your server run the following snippet, adjusting the `server-ip` and `socket-ip` parameters as needed.

```bash
curl -sL https://raw.githubusercontent.com/AlvaroParker/lab-control/main/install | bash -s -- --server-ip <ip> --socket-ip <ip>
```

This will install the server and start the docker service. To access the web interface go to the ip you pass as next to `--server-ip`. The inital admin is `first@admin.com` and the password `admin`.

## Alternative

Download the latest `deploy.zip` from the Releases website on Github. You can configure the ip of your server and the socket fingerprint device using the `changeip` script inside the `deploy` zip.

You can configure the database password using the `.env` file inside `deploy/`

To deploy the server just type:

```
docker compose up
```

To deploy the fingerprint device, you'll need to connect the fingerprint device to a Linux machine and run `fprs` binaries available in the release page. There are you compiled targets, for `armv7` and `x86_64` architectures.

# Development build

_Note: To acess the local website you won't be able to use `localhost`, you'll need to access using your local ip_

- To run this in development mode, you'll need `docker` engine, `cc` and `openssl` installed.
- Configuration of the server ip and fingerprint socket ip can be set on `deploy/.env`
- Valid `.env` variables:

```dotenv
# Where is the socket of the fingerprint devices is running
SOCKET_IP=192.168.68.112
SOCKET_PORT=8001

# Postgres password, user, and db.
PG_PASSWORD=this_is_a_super_secret_password
PG_USER=postgres
PG_DB=fingerprints
DATABASE_URL=postgres://${PG_USER}:${PG_PASSWORD}@database/${PG_DB}

# What's the ip of your backend? For LAN development check your ip using `ip a`
VITE_BACKEND_IP=192.168.68.112
VITE_BACKEND_PORT=8001
```

After properly setting this `env` variables, you are good to go. In the root directory of the project run

```bash
docker compose up
```

You will be able to visit the frontend by visiting the provided `VITE_BACKEND_IP` since both the backend and the frontend are running under a `nginx` container that implements only https.

Once the development containers are deployed, when you make changes in `backend/` or `frontend/` the container will automatically refresh the files and update the `frontend` or the `backend` accordingly

# Production build

In order to build the production backend you'll need `cc`, `openssl` and `openssl-devel` installed in your system.

Configure the `.env` variables properly. Then run `./build` and your deployment files will be under the `deploy/` directory. After this, you can copy this directory and deploy it in any server
by running `docker compose up`

_Note: to copy a docker volume: `cp -a /var/lib/docker/volumes/source_volume /var/lib/docker/volumes/target_volume` and `docker volume create target_volume`_
