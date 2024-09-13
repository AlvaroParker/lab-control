# Automated Lab control release

Files:

- `deploy.zip` Server deployment zip file. Unzip on the target server and run `docker compose up`
- `fprs-armv7` Sensor socket server. Built for `armv7` cpus.
- `fprs-x86` Sensor socket server. Built for `x86` cpus.

You can deploy this release on your server by executing the following command:

```
curl -sL https://raw.githubusercontent.com/AlvaroParker/lab-control/main/install | bash -s -- --server-ip <ip> --socket-ip <ip>
```

You'll need `docker`, `docker-compose`, `wget` and `unzip` installed on your server for the script to work.