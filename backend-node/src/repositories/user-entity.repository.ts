import {inject} from '@loopback/core';
import {DefaultCrudRepository} from '@loopback/repository';
import {DbDataSource} from '../datasources';
import {UserEntity, UserEntityRelations} from '../models';

export class UserEntityRepository extends DefaultCrudRepository<
  UserEntity,
  typeof UserEntity.prototype.id,
  UserEntityRelations
  > {
  constructor(
    @inject('datasources.db') dataSource: DbDataSource,
  ) {
    super(UserEntity, dataSource);
  }

  public async findByName(name: string): Promise<UserEntity | null> {
    return super.findOne({where: {fullname: name}});
  }
}
