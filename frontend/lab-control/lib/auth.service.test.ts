import { Login } from './auth.service'
import MockAdapter from 'axios-mock-adapter'
import { Status, axiosInstace as axios } from '.'

describe('AuthService.Login()', () => {
    const mock = new MockAdapter(axios)

    it('Should login OK', async () => {
        const responseData = {
            apellido_1: 'Doe',
            apellido_2: 'Smith',
            email: 'some@email.com',
            nombre: 'John',
            token: 'someToken',
        }
        mock.onPost('/admin/login').reply(200, responseData, {
            'set-cookie': 'someCookie',
        })

        const [adminData, status] = await Login(
            'some@email.com',
            'SomePassword'
        )
        expect(status).toEqual(Status.OK)
        expect(adminData).toEqual(responseData)
    })

    it('Should return BAD_REQUEST', async () => {
        mock.onPost('/admin/login').reply(400)

        const [adminData, status] = await Login(
            'some@email.com',
            'SomePassword'
        )
        expect(status).toEqual(Status.BAD_REQUEST)
        expect(adminData).toBeNull()
    })

    it('Should return UNAUTHORIZED', async () => {
        mock.onPost('/admin/login').reply(401)

        const [adminData, status] = await Login(
            'some@email.com',
            'SomePassword'
        )

        expect(status).toEqual(Status.UNAUTHORIZED)
        expect(adminData).toBeNull()
    })

    it('Should return NOT_FOUND', async () => {
        mock.onPost('/admin/login').reply(404)

        const [adminData, status] = await Login(
            'some@email.com',
            'SomePassword'
        )

        expect(status).toEqual(Status.NOT_FOUND)
        expect(adminData).toBeNull()
    })

    it('Should return INTERNAL_SERVER_ERROR', async () => {
        mock.onPost('/admin/login').reply(500)

        const [adminData, status] = await Login(
            'some@email.com',
            'SomePassword'
        )

        expect(status).toEqual(Status.INTERNAL_SERVER_ERROR)
        expect(adminData).toBeNull()
    })
})
