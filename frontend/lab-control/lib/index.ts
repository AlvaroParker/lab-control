import * as AuthService from './auth.service'
import * as DeleteService from './delete.service'
import * as GetService from './get.service'
import * as PostService from './post.service'
import * as ServiceTypes from './types'
import { Status } from './types'

export let IP = '192.168.68.115'
export let PORT = '8080'
export let HTTPS = true

export function getPort() {
    return PORT
}
export function getIP() {
    return IP
}

export function setHTTPS(https: boolean) {
    HTTPS = https
}

export function getURL() {
    if (HTTPS) return `https://${IP}:${PORT}/api`
    return `http://${IP}:${PORT}/api`
}

export function getWSURL() {
    if (HTTPS) return `wss://${IP}:${PORT}/api`
    return `ws://${IP}:${PORT}/api`
}

export function setIPandPort(ip: string, port: string) {
    IP = ip
    PORT = port
}

export {
    /**
     * This export contains all the functions that handle the authentication of the user.
     * Possible responses (`Status`) are:
     * - `Status.OK` (200)
     * - `Status.BAD_REQUEST` (400)
     * - `Status.UNAUTHORIZED` (401)
     * - `Status.NOT_FOUND` (404)
     * - `Status.INTERNAL_SERVER_ERROR` (500)
     *
     * You should try to handle the `Status` of the request to give feedback to the user.
     */
    AuthService,

    /**
     * This export contains all the functions that make DELETE requests to the server.
     * Possible responses (`Status`) are:
     * - `Status.OK` (200)
     * - `Status.BAD_REQUEST` (400)
     * - `Status.UNAUTHORIZED` (401)
     * - `Status.NOT_FOUND` (404)
     * - `Status.INTERNAL_SERVER_ERROR` (500)
     *
     * You should try to handle the `Status` of the request to give feedback to the user.
     */
    DeleteService,

    /**
     * This export contains all the functions that make GET requests to the server.
     * Possible responses (`Status`) are:
     * - `Status.OK` (200)
     * - `Status.BAD_REQUEST` (400)
     * - `Status.UNAUTHORIZED` (401)
     * - `Status.NOT_FOUND` (404)
     * - `Status.INTERNAL_SERVER_ERROR` (500)
     *
     * You should try to handle the `Status` of the request to give feedback to the user.
     */
    GetService,

    /**
     * This export contains all the functions that make POST requests to the server.
     * Possible responses (`Status`) are:
     * - `Status.OK` (200)
     * - `Status.BAD_REQUEST` (400)
     * - `Status.UNAUTHORIZED` (401)
     * - `Status.NOT_FOUND` (404)
     * - `Status.CONFLICT` (409)
     * - `Status.INTERNAL_SERVER_ERROR` (500)
     *
     * You should try to handle the `Status` of the request to give feedback to the user.
     */
    PostService,
    ServiceTypes,
    Status,
}
