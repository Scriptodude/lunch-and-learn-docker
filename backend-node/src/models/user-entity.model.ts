import {Entity, model, property} from '@loopback/repository';

@model({
  settings: {strict: false},
  name: 'users'
})
export class UserEntity extends Entity {
  @property({id: true})
  id: number;

  @property({required: true})
  fullname: string;

  constructor(data?: Partial<UserEntity>) {
    super(data);
  }
}

export interface UserEntityRelations {
  // describe navigational properties here
}

export type UserDaoWithRelations = UserEntity & UserEntityRelations;
