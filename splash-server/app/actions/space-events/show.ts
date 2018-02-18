import { lookup } from 'denali';
import ApplicationAdapter from '../../orm-adapters/typeorm';
import ApplicationAction from '../application';

export default class ShowSpaceEvent extends ApplicationAction {

  adapter = lookup<ApplicationAdapter>('adapter:space-event')

  async respond({ params } : any) {
    return await this.adapter.find('space-event', params.id);
  }
}
