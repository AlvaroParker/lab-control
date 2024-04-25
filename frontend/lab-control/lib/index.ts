import AuthService from "./auth.service";
import DeleteService from "./delete.service";
import GetService from "./get.service";
import PostService from "./post.service";
import ServiceTypes from './types'
import { Status } from "./types";

export let IP = "192.168.68.115"
export let PORT = "8080"
export let HTTPS = true

export function getPort() {
  return PORT
}
export function getIP() {
  return IP
}

export function setHTTPS(https: boolean) {
  HTTPS = https;
}

export function getURL() {
  if (HTTPS)
    return `https://${IP}:${PORT}/api`
  return `http://${IP}:${PORT}/api`
}

export function getWSURL() {
  if (HTTPS)
    return `wss://${IP}:${PORT}/api`
  return `ws://${IP}:${PORT}/api`
}

export function setIPandPort(ip: string, port: string) {
  IP = ip;
  PORT = port;
}

export {
  AuthService,
  DeleteService,
  GetService,
  PostService,
  ServiceTypes,
  Status
}
