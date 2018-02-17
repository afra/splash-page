import ApplicationAction from '../application';
import SpaceEvent from '../../models/space-event';

export default class CreateSpaceEvent extends ApplicationAction {

  async respond({ body }) {
    let spaceEvent = await SpaceEvent.create(body);
    this.render(201, spaceEvent);
  }

}
