import ApplicationAction from '../application';
import SpaceEvent from '../../models/space-event';

export default class ListSpaceEvents extends ApplicationAction {

  async respond() {
    return await SpaceEvent.all();
  }

}
