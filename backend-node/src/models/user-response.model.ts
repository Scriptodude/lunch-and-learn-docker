import { ResponseObject } from '@loopback/rest';

export default class UserResponse implements ResponseObject {
    constructor(public id: Number, public name: String) { }


    description: "The user object";
    content?: {
        'application/json': {
            schema: {
                type: 'object',
                title: 'UserResponse',
                properties: {
                    id: { type: 'number' },
                    name: { type: 'string' },
                },
            },
        },
    }
}