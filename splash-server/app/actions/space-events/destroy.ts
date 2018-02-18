import {lookup} from 'denali';
import TypeOrmAdapter from '../../orm-adapters/typeorm';
import ApplicationAction from '../application';

export default class DestroySpaceEvent extends ApplicationAction {

  adapter = lookup<TypeOrmAdapter>('adapter:space-event')

  async respond({ params } : any) {
    const spaceEvent = await this.adapter.find('space-event', params.id);
    await this.adapter.deleteRecord(spaceEvent);
    this.render(204);
  }

}
