import {lookup} from 'denali';
import TypeOrmAdapter from '../../orm-adapters/typeorm';
import ApplicationAction from '../application';

export default class CreateSpaceEvent extends ApplicationAction {

  adapter = lookup<TypeOrmAdapter>('adapter:space-event')

  async respond({ body } : any) {
    const spaceEvent = this.adapter.buildRecord('space-event', body);
    await this.adapter.saveRecord(spaceEvent);
    this.render(201, spaceEvent);
  }
}
