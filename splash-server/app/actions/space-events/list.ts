import {lookup} from 'denali';
import TypeOrmAdapter from '../../orm-adapters/typeorm';
import ApplicationAction from '../application';
// import {SpaceEvent} from '../../models/space-event'
// import {getManager} from "typeorm";

export default class ListSpaceEvents extends ApplicationAction {

  adapter = lookup<TypeOrmAdapter>('orm-adapter:application')

  async respond({ params } : any) {
    return await this.adapter.all('space-event'); // Todo: is this an SQL-injection?
  }

}
