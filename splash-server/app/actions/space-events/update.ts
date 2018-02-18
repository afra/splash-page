import { lookup } from 'denali';
import TypeOrmAdapter from '../../orm-adapters/typeorm';
import ApplicationAction from '../application';

export default class UpdateSpaceEvent extends ApplicationAction {

  adapter = lookup<TypeOrmAdapter>('orm-adapter:application')

  async respond({ params, body } : any) {
    const spaceEvent = await this.adapter.find('space-event', params.id);
    Object.assign(spaceEvent, body);
    await this.adapter.saveRecord(spaceEvent);
    return spaceEvent;
  }
}
