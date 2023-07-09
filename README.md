# Development build

- To run this in development mode, you'll need `docker` engine installed.
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

You will be able to visit the frontend by visiting the provided `VITE_BACKEND_IP` since both the backend and the frontend is running under a `nginx` container that implements only https

# Production build

Configure the `.env` variables properly. Then run `./install` and your deployment file will be under the `deploy/` directory. After this, you can copy this directory and deploy it in any server
by running `docker compose up`
