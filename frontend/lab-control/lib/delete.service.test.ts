import MockAdapter from 'axios-mock-adapter'
import { DeleteService, Status } from '.'
import { axiosInstace as axios } from '.'

const mock = new MockAdapter(axios)

describe('DeleteService.DeleteUsuario()', () => {
    it('Should login OK', async () => {
        mock.onDelete('/usuarios/123456789').reply(200)

        const status1 = await DeleteService.DeleteUsuario('123456789')
        const status2 = await DeleteService.DeleteUsuario('12345678-9')
        const status3 = await DeleteService.DeleteUsuario('12.345.678-9')

        expect(status1).toEqual(Status.OK)
        expect(status2).toEqual(Status.OK)
        expect(status3).toEqual(Status.OK)
    })

    it('Should return NOT_FOUND', async () => {
        mock.onDelete('/usuarios/123456789').reply(404)

        const status = await DeleteService.DeleteUsuario('123456789')

        expect(status).toEqual(Status.NOT_FOUND)
    })

    it('Should return UNAUTHORIZED', async () => {
        mock.onDelete('/usuarios/123456789').reply(401)

        const status = await DeleteService.DeleteUsuario('123456789')

        expect(status).toEqual(Status.UNAUTHORIZED)
    })

    it('Should return INTERNAL_SERVER_ERROR', async () => {
        mock.onDelete('/usuarios/123456789').reply(500)

        const status = await DeleteService.DeleteUsuario('123456789')

        expect(status).toEqual(Status.INTERNAL_SERVER_ERROR)
    })

    it('Should return BAD_REQUEST', async () => {
        mock.onDelete('/usuarios/123456789').reply(400)

        const status = await DeleteService.DeleteUsuario('123456789')

        expect(status).toEqual(Status.BAD_REQUEST)
    })

    it('Should return UNKNOWN', async () => {
        mock.onDelete('/usuarios/123456789').reply(420)

        const status = await DeleteService.DeleteUsuario('123456789')

        expect(status).toEqual(Status.UNKNOWN)
    })
})

describe('DeleteService.DeleteMotivo()', () => {
    it('Should delete OK', async () => {
        mock.onDelete('/metadata/motivos/1').reply(200)

        const status = await DeleteService.DeleteMotivo(1)

        expect(status).toEqual(Status.OK)
    })

    it('Should return NOT_FOUND', async () => {
        mock.onDelete('/metadata/motivos/1').reply(404)

        const status = await DeleteService.DeleteMotivo(1)

        expect(status).toEqual(Status.NOT_FOUND)
    })

    it('Should return UNAUTHORIZED', async () => {
        mock.onDelete('/metadata/motivos/1').reply(401)

        const status = await DeleteService.DeleteMotivo(1)

        expect(status).toEqual(Status.UNAUTHORIZED)
    })

    it('Should return INTERNAL_SERVER_ERROR', async () => {
        mock.onDelete('/metadata/motivos/1').reply(500)

        const status = await DeleteService.DeleteMotivo(1)

        expect(status).toEqual(Status.INTERNAL_SERVER_ERROR)
    })

    it('Should return BAD_REQUEST', async () => {
        mock.onDelete('/metadata/motivos/1').reply(400)

        const status = await DeleteService.DeleteMotivo(1)

        expect(status).toEqual(Status.BAD_REQUEST)
    })
})

describe('DeleteService.DeleteRol()', () => {
    it('Should delete OK', async () => {
        mock.onDelete('/metadata/roles/1').reply(200)

        const status = await DeleteService.DeleteRol(1)

        expect(status).toEqual(Status.OK)
    })

    it('Should return NOT_FOUND', async () => {
        mock.onDelete('/metadata/roles/1').reply(404)

        const status = await DeleteService.DeleteRol(1)

        expect(status).toEqual(Status.NOT_FOUND)
    })

    it('Should return UNAUTHORIZED', async () => {
        mock.onDelete('/metadata/roles/1').reply(401)

        const status = await DeleteService.DeleteRol(1)

        expect(status).toEqual(Status.UNAUTHORIZED)
    })

    it('Should return INTERNAL_SERVER_ERROR', async () => {
        mock.onDelete('/metadata/roles/1').reply(500)

        const status = await DeleteService.DeleteRol(1)

        expect(status).toEqual(Status.INTERNAL_SERVER_ERROR)
    })

    it('Should return BAD_REQUEST', async () => {
        mock.onDelete('/metadata/roles/1').reply(400)

        const status = await DeleteService.DeleteRol(1)

        expect(status).toEqual(Status.BAD_REQUEST)
    })
})

// Todo: Should probably change the delete request to use id's instead of emails,
// to do this, we need to change the return tpye of the GetAdmins() function to also return
// the id of the admin, so we can use it to delete the admin.
describe('DeleteService.DeleteAdmin()', () => {
    it('Should delete OK', async () => {
    })
})
