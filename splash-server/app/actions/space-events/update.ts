import { lookup } from 'denali';
import ApplicationAdapter from '../../orm-adapters/typeorm';
import ApplicationAction from '../application';

export default class UpdateSpaceEvent extends ApplicationAction {

  adapter = lookup<ApplicationAdapter>('adapter:space-event')

  async respond({ params, body } : any) {
    const spaceEvent = await this.adapter.find('space-event', params.id);
    Object.assign(spaceEvent, body);
    await this.adapter.saveRecord(spaceEvent);
    return spaceEvent;
  }
}
