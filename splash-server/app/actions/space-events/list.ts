import {lookup} from 'denali';
import TypeOrmAdapter from '../../orm-adapters/typeorm';
import ApplicationAction from '../application';

export default class ListSpaceEvents extends ApplicationAction {

  adapter = lookup<TypeOrmAdapter>('orm-adapter:application')

  async respond({ params } : any) {
    return await this.adapter.query('space-event'); // Todo: is this an SQL-injection?
  }

}
