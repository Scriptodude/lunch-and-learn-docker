import {inject} from '@loopback/core';
import {repository} from '@loopback/repository';
import {api, get, param, Response, RestBindings} from '@loopback/rest';
import {UserEntity} from '../models';
import UserResponse from '../models/user-response.model';
import {UserEntityRepository} from '../repositories';

@api({basePath: "/api/hello"})
export class UserController {
  constructor(
    @inject(RestBindings.Http.RESPONSE) private response: Response,
    @repository(UserEntityRepository) private userRepo: UserEntityRepository) {}

  @get("/user/{id}/{name}", {
    responses: {
      '200': UserResponse
    }
  })
  public async getUserByIdAndName(
    @param.path.number('id') id: number,
    @param.path.string('name') name: string,
  ) {
    this.response.setHeader("ACCESS_CONTROL_ALLOW_ORIGIN", "*");
    return new UserResponse(id, name);
  }

  @get("/name/{name}", {
    responses: {
      '200': UserResponse
    }
  })
  public async getUserByName(
    @param.path.string('name') name: string,
  ) {
    const user = await this.userRepo.findByName(name);
    this.response.setHeader("ACCESS_CONTROL_ALLOW_ORIGIN", "*");
    return new UserResponse(user?.id ?? -1, user?.fullname ?? 'Unkown');
  }

  @get("/id/{id}", {
    responses: {
      '200': UserResponse
    }
  })
  public async getById(
    @param.path.number('id') id: number
  ) {
    const user: UserEntity = await this.userRepo.findById(id);
    this.response.setHeader("ACCESS_CONTROL_ALLOW_ORIGIN", "*");
    return new UserResponse(user.id, user.fullname);
  }
}
